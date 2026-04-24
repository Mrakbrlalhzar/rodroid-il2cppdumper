mod arm;
mod x86;

use std::collections::{HashMap, HashSet, BTreeMap};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Architecture {
    Arm32,
    Arm64,
    X86,
    X64,
}

impl fmt::Display for Architecture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Architecture::Arm32 => write!(f, "ARM32"),
            Architecture::Arm64 => write!(f, "ARM64"),
            Architecture::X86 => write!(f, "x86"),
            Architecture::X64 => write!(f, "x86_64"),
        }
    }
}

impl Architecture {
    pub fn from_elf_machine(e_machine: u16) -> Option<Self> {
        match e_machine {
            40 => Some(Architecture::Arm32),
            183 => Some(Architecture::Arm64),
            3 => Some(Architecture::X86),
            62 => Some(Architecture::X64),
            _ => None,
        }
    }

    pub fn from_pe_machine(machine: u16) -> Option<Self> {
        match machine {
            0x14C => Some(Architecture::X86),
            0x8664 => Some(Architecture::X64),
            0x1C0 | 0x1C4 => Some(Architecture::Arm32),
            0xAA64 => Some(Architecture::Arm64),
            _ => None,
        }
    }

    pub fn from_macho_cputype(cputype: u32) -> Option<Self> {
        match cputype {
            12 => Some(Architecture::Arm32),
            0x0100_000C => Some(Architecture::Arm64),
            7 => Some(Architecture::X86),
            0x0100_0007 => Some(Architecture::X64),
            _ => None,
        }
    }

    pub fn from_bitness(is_32bit: bool, is_pe: bool) -> Self {
        if is_pe {
            if is_32bit { Architecture::X86 } else { Architecture::X64 }
        } else {
            if is_32bit { Architecture::Arm32 } else { Architecture::Arm64 }
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisassembledInstruction {
    pub address: u64,
    pub size: usize,
    pub raw_bytes: Vec<u8>,
    pub mnemonic: String,
    pub operands: String,
    pub is_call: bool,
    pub is_return: bool,
    pub is_branch: bool,
    pub is_unconditional_branch: bool,
    pub call_target: Option<u64>,
    pub branch_target: Option<u64>,
    pub condition_code: Option<String>,
    pub memory_offset: Option<i64>,
}

impl fmt::Display for DisassembledInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.operands.is_empty() {
            write!(f, "{}", self.mnemonic)
        } else {
            write!(f, "{} {}", self.mnemonic, self.operands)
        }
    }
}

pub struct DisassemblyContext {
    pub field_offsets: HashMap<i32, String>,
    pub string_literals: HashMap<u64, String>,
    pub type_names: HashMap<u64, String>,
    pub method_refs: HashMap<u64, String>,
    pub field_refs: HashMap<u64, String>,
    pub vtable_methods: HashMap<i32, String>,
    pub register_names: HashMap<String, String>,
}

impl DisassemblyContext {
    pub fn new() -> Self {
        Self {
            field_offsets: HashMap::new(),
            string_literals: HashMap::new(),
            type_names: HashMap::new(),
            method_refs: HashMap::new(),
            field_refs: HashMap::new(),
            vtable_methods: HashMap::new(),
            register_names: HashMap::new(),
        }
    }

    pub fn resolve_register(&self, reg: &str) -> String {
        let lower = reg.to_lowercase();
        if let Some(name) = self.register_names.get(&lower) {
            return name.clone();
        }
        reg.to_string()
    }

    pub fn resolve_operands(&self, operands: &str) -> String {
        let mut result = operands.to_string();
        for (reg, name) in &self.register_names {
            let upper = reg.to_uppercase();
            let patterns = [
                format!("{}, ", upper),
                format!("{},", upper),
                format!("[{},", upper),
                format!("[{}]", upper),
                format!("{} ", upper),
            ];
            for pat in &patterns {
                if result.contains(pat.as_str()) {
                    result = result.replace(pat.as_str(), &pat.replace(&upper, name));
                }
            }
        }
        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetadataAnnotationKind {
    StringLiteral,
    TypeInfo,
    MethodRef,
    FieldRef,
}

#[derive(Debug, Clone)]
pub struct MetadataAnnotation {
    pub kind: MetadataAnnotationKind,
    pub label: String,
}

pub struct Disassembler {
    arch: Architecture,
    rva_to_name: HashMap<u64, String>,
    global_annotations: HashMap<u64, MetadataAnnotation>,
}

impl Disassembler {
    pub fn new(arch: Architecture) -> Self {
        Self {
            arch,
            rva_to_name: HashMap::new(),
            global_annotations: HashMap::new(),
        }
    }

    pub fn arch(&self) -> Architecture {
        self.arch
    }
    pub fn set_method_names(&mut self, map: HashMap<u64, String>) {
        self.rva_to_name = map;
    }

    pub fn annotation_count(&self) -> usize {
        self.global_annotations.len()
    }

    pub fn add_string_literal(&mut self, rva: u64, value: String) {
        let escaped = value
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('\t', "\\t")
            .replace('\0', "\\0");
        let truncated = if escaped.len() > 60 {
            let safe_end = escaped.char_indices()
                .map(|(i, _)| i)
                .take_while(|&i| i <= 57)
                .last()
                .unwrap_or(0);
            format!("{}...", &escaped[..safe_end])
        } else {
            escaped
        };
        self.global_annotations.insert(rva, MetadataAnnotation {
            kind: MetadataAnnotationKind::StringLiteral,
            label: format!("\"{}\"", truncated),
        });
    }

    pub fn add_type_info(&mut self, rva: u64, type_name: String) {
        self.global_annotations.insert(rva, MetadataAnnotation {
            kind: MetadataAnnotationKind::TypeInfo,
            label: format!("typeof({})", type_name),
        });
    }

    pub fn add_method_ref(&mut self, rva: u64, full_name: String) {
        self.global_annotations.insert(rva, MetadataAnnotation {
            kind: MetadataAnnotationKind::MethodRef,
            label: full_name,
        });
    }

    pub fn add_field_ref(&mut self, rva: u64, full_name: String) {
        self.global_annotations.insert(rva, MetadataAnnotation {
            kind: MetadataAnnotationKind::FieldRef,
            label: full_name,
        });
    }

    pub fn disassemble(&self, bytes: &[u8], base_address: u64, max_instructions: usize) -> Vec<DisassembledInstruction> {
        match self.arch {
            Architecture::Arm64 => arm::disassemble_arm64(bytes, base_address, max_instructions),
            Architecture::Arm32 => arm::disassemble_arm32(bytes, base_address, max_instructions),
            Architecture::X86 => x86::disassemble_x86(bytes, base_address, max_instructions, 32),
            Architecture::X64 => x86::disassemble_x86(bytes, base_address, max_instructions, 64),
        }
    }

    pub fn format_method_body(
        &self,
        bytes: &[u8],
        base_address: u64,
        max_instructions: usize,
        indent: &str,
        ctx: Option<&DisassemblyContext>,
        show_hex_bytes: bool,
        show_field_names: bool,
        show_annotations: bool,
        show_cfg: bool,
    ) -> String {
        let instructions = self.disassemble(bytes, base_address, max_instructions);
        if instructions.is_empty() {
            return String::new();
        }

        let total_bytes: usize = instructions.iter().map(|i| i.size).sum();

        let reg_names = ctx.map(|c| &c.register_names);

        let cfg = if show_cfg {
            Some(CfgAnalysis::build(&instructions, reg_names))
        } else {
            None
        };

        let mut buf = String::with_capacity(instructions.len() * 120);
        buf.push_str(&format!(
            "{indent}\t\t/* Disassembly ({}, {} instructions, 0x{:X} bytes):\n",
            self.arch,
            instructions.len(),
            total_bytes,
        ));

        if let Some(ref cfg) = cfg {
            if !cfg.blocks.is_empty() {
                buf.push_str(&format!(
                    "{indent}\t\t   // CFG: {} blocks, {} branches",
                    cfg.blocks.len(),
                    cfg.edge_count,
                ));
                if cfg.loop_count > 0 {
                    buf.push_str(&format!(", {} loop(s)", cfg.loop_count));
                }
                buf.push('\n');
            }
        }

        let mut adrp_page: Option<u64> = None;
        let mut _last_cmp_operands: Option<String> = None;

        for (idx, insn) in instructions.iter().enumerate() {
            if let Some(ref cfg) = cfg {
                if let Some(block_header) = cfg.block_headers.get(&insn.address) {
                    buf.push_str(&format!(
                        "{indent}\t\t   // {}\n",
                        block_header,
                    ));
                }
            }

            if show_hex_bytes {
                let hex = format_hex_bytes(&insn.raw_bytes);
                buf.push_str(&format!(
                    "{indent}\t\t   0x{:08X}:  {:<12} {}",
                    insn.address, hex, insn
                ));
            } else {
                buf.push_str(&format!(
                    "{indent}\t\t   0x{:08X}:  {}",
                    insn.address, insn
                ));
            }

            let mut annotated = false;

            if show_annotations {
                if let Some(target) = insn.call_target {
                    if let Some(name) = self.rva_to_name.get(&target) {
                        buf.push_str(&format!("  // CALL → {name}"));
                        annotated = true;
                    } else if let Some(ann) = self.global_annotations.get(&target) {
                        buf.push_str(&format!("  // CALL → {}", ann.label));
                        annotated = true;
                    } else {
                        buf.push_str(&format!("  // CALL → sub_{:X}", target));
                        annotated = true;
                    }
                }
            }

            if !annotated && insn.is_branch && !insn.is_call && !insn.is_return {
                if let Some(target) = insn.branch_target {
                    if insn.is_unconditional_branch {
                        if let Some(name) = self.rva_to_name.get(&target) {
                            buf.push_str(&format!("  // TAIL CALL → {name}"));
                            annotated = true;
                        }
                    }
                }
                if !annotated {
                    if let Some(ref cfg) = cfg {
                        if let Some(edge_label) = cfg.edge_labels.get(&insn.address) {
                            buf.push_str(&format!("  // {}", edge_label));
                            annotated = true;
                        }
                    }
                }
            }

            if !annotated && show_annotations && insn.mnemonic == "ADRP" {
                let page = extract_adrp_page(insn);
                adrp_page = page;
            }

            if !annotated && show_annotations && (insn.mnemonic == "LDR" || insn.mnemonic == "ADD") {
                if let Some(page_base) = adrp_page {
                    if let Some(offset) = insn.memory_offset {
                        let full_addr = page_base.wrapping_add(offset as u64);
                        if let Some(ann) = self.global_annotations.get(&full_addr) {
                            let prefix = match ann.kind {
                                MetadataAnnotationKind::StringLiteral => "str",
                                MetadataAnnotationKind::TypeInfo => "type",
                                MetadataAnnotationKind::MethodRef => "method",
                                MetadataAnnotationKind::FieldRef => "field",
                            };
                            buf.push_str(&format!("  // {}: {}", prefix, ann.label));
                            annotated = true;
                        }
                    }
                }

                if !annotated {
                    if let Some(offset) = insn.memory_offset {
                        if let Some(ann) = self.global_annotations.get(&(offset as u64)) {
                            let prefix = match ann.kind {
                                MetadataAnnotationKind::StringLiteral => "str",
                                MetadataAnnotationKind::TypeInfo => "type",
                                MetadataAnnotationKind::MethodRef => "method",
                                MetadataAnnotationKind::FieldRef => "field",
                            };
                            buf.push_str(&format!("  // {}: {}", prefix, ann.label));
                            annotated = true;
                        }
                    }
                }
            }

            if !annotated && show_field_names {
                if let Some(offset) = insn.memory_offset {
                    if let Some(ctx) = ctx {
                        let operand_text = &insn.operands;
                        let is_sp_access = operand_text.contains("sp,")
                            || operand_text.contains("sp]")
                            || operand_text.contains("SP,")
                            || operand_text.contains("SP]");

                        if !is_sp_access {
                            if let Some(field_name) = ctx.field_offsets.get(&(offset as i32)) {
                                if !insn.is_call {
                                    buf.push_str(&format!("  // this.{field_name}"));
                                    annotated = true;
                                }
                            }
                        }

                        if !annotated && !is_sp_access {
                            if let Some(vtable_method) = ctx.vtable_methods.get(&(offset as i32)) {
                                buf.push_str(&format!("  // vtable: {vtable_method}"));
                            }
                        }
                    }
                }
            }

            if insn.mnemonic == "CMP" || insn.mnemonic == "TST"
                || insn.mnemonic == "TEST" || insn.mnemonic == "SUBS"
                || insn.mnemonic == "CCMP"
            {
                _last_cmp_operands = Some(insn.operands.clone());
            }

            buf.push('\n');

            if let Some(ref cfg) = cfg {
                if insn.is_branch && !insn.is_call {
                    if let Some(separator) = cfg.block_separators.get(&insn.address) {
                        buf.push_str(&format!(
                            "{indent}\t\t   // {}\n",
                            separator,
                        ));
                    }
                }
            }

            if insn.is_unconditional_branch && !insn.is_call {
                if idx + 1 < instructions.len() {
                    if cfg.is_none() {
                        buf.push('\n');
                    }
                }
            }
        }

        buf.push_str(&format!("{indent}\t\t*/\n"));
        buf
    }
}

struct CfgAnalysis {
    blocks: Vec<BlockInfo>,
    block_headers: HashMap<u64, String>,
    block_separators: HashMap<u64, String>,
    edge_labels: HashMap<u64, String>,
    edge_count: usize,
    loop_count: usize,
}

struct BlockInfo {
    _start_addr: u64,
    _end_addr: u64,
    _id: usize,
}

impl CfgAnalysis {
    fn build(instructions: &[DisassembledInstruction], reg_names: Option<&HashMap<String, String>>) -> Self {
        if instructions.is_empty() {
            return Self {
                blocks: Vec::new(),
                block_headers: HashMap::new(),
                block_separators: HashMap::new(),
                edge_labels: HashMap::new(),
                edge_count: 0,
                loop_count: 0,
            };
        }

        let addr_set: HashSet<u64> = instructions.iter().map(|i| i.address).collect();
        let first_addr = instructions[0].address;
        let last_addr = instructions.last().unwrap().address;

        let mut branch_targets: BTreeMap<u64, Vec<IncomingEdge>> = BTreeMap::new();

        let mut last_cmp: Option<String> = None;

        for (_idx, insn) in instructions.iter().enumerate() {
            if insn.mnemonic == "CMP" || insn.mnemonic == "TST"
                || insn.mnemonic == "TEST" || insn.mnemonic == "SUBS"
                || insn.mnemonic == "CCMP"
            {
                last_cmp = Some(resolve_operands_with_names(&insn.operands, reg_names));
            }

            if !insn.is_branch || insn.is_call {
                continue;
            }

            if let Some(target) = insn.branch_target {
                if !addr_set.contains(&target) {
                    continue;
                }

                let is_back_edge = target <= insn.address;

                let condition_text = build_condition_text(insn, &last_cmp, is_back_edge, reg_names);

                branch_targets
                    .entry(target)
                    .or_insert_with(Vec::new)
                    .push(IncomingEdge {
                        from_addr: insn.address,
                        condition: condition_text,
                        is_back_edge,
                    });
            }

            if insn.condition_code.is_some() && !insn.is_unconditional_branch {
                let fall_through = insn.address + insn.size as u64;
                if addr_set.contains(&fall_through) {
                    let negated = negate_condition_text(insn, &last_cmp, reg_names);
                    branch_targets
                        .entry(fall_through)
                        .or_insert_with(Vec::new)
                        .push(IncomingEdge {
                            from_addr: insn.address,
                            condition: negated,
                            is_back_edge: false,
                        });
                }
            }
        }

        let mut block_starts: HashSet<u64> = HashSet::new();
        block_starts.insert(first_addr);
        for target in branch_targets.keys() {
            block_starts.insert(*target);
        }
        for insn in instructions {
            if insn.is_branch && !insn.is_call {
                let after = insn.address + insn.size as u64;
                if addr_set.contains(&after) {
                    block_starts.insert(after);
                }
            }
        }

        let mut sorted_starts: Vec<u64> = block_starts.into_iter().collect();
        sorted_starts.sort_unstable();

        let mut blocks: Vec<BlockInfo> = Vec::new();
        for (id, &start) in sorted_starts.iter().enumerate() {
            let end = sorted_starts.get(id + 1).copied()
                .unwrap_or(last_addr + instructions.last().unwrap().size as u64);
            blocks.push(BlockInfo {
                _start_addr: start,
                _end_addr: end,
                _id: id,
            });
        }

        let mut edge_count = 0;
        let mut loop_count = 0;
        let mut block_headers: HashMap<u64, String> = HashMap::new();
        let mut block_separators: HashMap<u64, String> = HashMap::new();
        let mut edge_labels: HashMap<u64, String> = HashMap::new();

        if blocks.len() > 1 {
            block_headers.insert(first_addr, format!(
                "═══ Block 0 (entry) {}",
                "═".repeat(30)
            ));
        }

        for (target_addr, edges) in &branch_targets {
            if *target_addr == first_addr { continue; }

            let block_idx = sorted_starts.iter().position(|a| *a == *target_addr);
            let block_label = block_idx.map(|i| format!("Block {}", i)).unwrap_or_default();

            let mut header_parts: Vec<String> = Vec::new();
            let mut has_back_edge = false;

            for edge in edges {
                edge_count += 1;
                if edge.is_back_edge {
                    has_back_edge = true;
                    loop_count += 1;
                }
                if !edge.condition.is_empty() {
                    header_parts.push(edge.condition.clone());
                }
            }

            if has_back_edge {
                let header = format!(
                    "──── ↑ loop target ({}) {} (from 0x{:X})",
                    header_parts.first().unwrap_or(&String::new()),
                    "─".repeat(20),
                    edges.iter().find(|e| e.is_back_edge).map(|e| e.from_addr).unwrap_or(0),
                );
                block_headers.insert(*target_addr, header);
            } else if header_parts.len() == 1 {
                let header = format!(
                    "──── {} {} {}",
                    block_label,
                    header_parts[0],
                    "─".repeat(20),
                );
                block_headers.insert(*target_addr, header);
            } else if header_parts.len() > 1 {
                let header = format!(
                    "──── {} (from {} paths) {}",
                    block_label,
                    header_parts.len(),
                    "─".repeat(18),
                );
                block_headers.insert(*target_addr, header);
            } else {
                let header = format!(
                    "──── {} {}",
                    block_label,
                    "─".repeat(30),
                );
                block_headers.insert(*target_addr, header);
            }
        }

        for insn in instructions {
            if !insn.is_branch || insn.is_call || insn.is_return { continue; }

            if let Some(target) = insn.branch_target {
                if !addr_set.contains(&target) { continue; }

                let is_back_edge = target <= insn.address;

                if is_back_edge {
                    edge_labels.insert(insn.address, format!("↑ loop back to 0x{:08X}", target));
                } else if insn.is_unconditional_branch {
                    edge_labels.insert(insn.address, format!("goto 0x{:08X}", target));
                } else if let Some(ref cond) = insn.condition_code {
                    let branch_label = build_branch_label(insn, cond, reg_names);
                    edge_labels.insert(insn.address, branch_label);
                }
            }

            if insn.condition_code.is_some() && !insn.is_unconditional_branch {
                let sep_addr = insn.address;
                if insn.branch_target.map(|t| t > insn.address).unwrap_or(false) {
                    block_separators.insert(sep_addr, String::new());
                }
            }
        }

        let last_block_entry = sorted_starts.last().copied().unwrap_or(0);
        let has_ret_in_last = instructions.iter().any(|i| i.is_return);
        if blocks.len() > 1 && has_ret_in_last {
            for insn in instructions.iter().rev() {
                if insn.is_return {
                    if insn.address >= last_block_entry && !block_headers.contains_key(&insn.address) {
                    }
                    break;
                }
            }
        }

        Self {
            blocks,
            block_headers,
            block_separators,
            edge_labels,
            edge_count,
            loop_count,
        }
    }
}

struct IncomingEdge {
    from_addr: u64,
    condition: String,
    is_back_edge: bool,
}

fn resolve_reg(reg: &str, reg_names: Option<&HashMap<String, String>>) -> String {
    if let Some(names) = reg_names {
        let lower = reg.to_lowercase().replace(' ', "");
        if let Some(name) = names.get(&lower) {
            return name.clone();
        }
    }
    reg.to_string()
}

fn resolve_operands_with_names(operands: &str, reg_names: Option<&HashMap<String, String>>) -> String {
    let names = match reg_names {
        Some(n) if !n.is_empty() => n,
        _ => return operands.to_string(),
    };

    let mut result = operands.to_string();
    for (reg_lower, name) in names {
        let reg_upper = reg_lower.to_uppercase();
        let lower = reg_lower.clone();

        let pats = [
            (format!("{}, ", reg_upper), format!("{}, ", name)),
            (format!("{},", reg_upper), format!("{},", name)),
            (format!("{}, ", lower), format!("{}, ", name)),
            (format!("{},", lower), format!("{},", name)),
        ];

        for (from, to) in &pats {
            result = result.replace(from.as_str(), to.as_str());
        }
    }
    result
}

fn build_condition_text(
    insn: &DisassembledInstruction,
    last_cmp: &Option<String>,
    _is_back_edge: bool,
    reg_names: Option<&HashMap<String, String>>,
) -> String {
    match insn.mnemonic.as_str() {
        "CBZ" => {
            let reg = insn.operands.split(',').next().unwrap_or("?").trim();
            let name = resolve_reg(reg, reg_names);
            format!("if ({} == null)", name)
        }
        "CBNZ" => {
            let reg = insn.operands.split(',').next().unwrap_or("?").trim();
            let name = resolve_reg(reg, reg_names);
            format!("if ({} != null)", name)
        }
        "TBZ" => {
            let parts: Vec<&str> = insn.operands.splitn(3, ',').collect();
            if parts.len() >= 2 {
                let reg = parts[0].trim();
                let name = resolve_reg(reg, reg_names);
                let bit = parts[1].trim().trim_start_matches('#');
                if bit == "0" {
                    format!("if (!{}.initialized)", name)
                } else {
                    format!("if (bit{} of {} == 0)", bit, name)
                }
            } else {
                "if (bit == 0)".to_string()
            }
        }
        "TBNZ" => {
            let parts: Vec<&str> = insn.operands.splitn(3, ',').collect();
            if parts.len() >= 2 {
                let reg = parts[0].trim();
                let name = resolve_reg(reg, reg_names);
                let bit = parts[1].trim().trim_start_matches('#');
                if bit == "0" {
                    format!("if ({}.initialized)", name)
                } else {
                    format!("if (bit{} of {} != 0)", bit, name)
                }
            } else {
                "if (bit != 0)".to_string()
            }
        }
        _ => {
            if let Some(ref cond) = insn.condition_code {
                if let Some(cmp_ops) = last_cmp {
                    let parts: Vec<&str> = cmp_ops.splitn(2, ',').collect();
                    if parts.len() == 2 {
                        let lhs = parts[0].trim();
                        let rhs = parts[1].trim();
                        format!("if ({} {} {})", lhs, cond, rhs)
                    } else {
                        format!("if ({})", cond)
                    }
                } else {
                    format!("if ({})", cond)
                }
            } else {
                String::new()
            }
        }
    }
}

fn negate_condition_text(
    insn: &DisassembledInstruction,
    last_cmp: &Option<String>,
    reg_names: Option<&HashMap<String, String>>,
) -> String {
    match insn.mnemonic.as_str() {
        "CBZ" => {
            let reg = insn.operands.split(',').next().unwrap_or("?").trim();
            let name = resolve_reg(reg, reg_names);
            format!("else ({} != null)", name)
        }
        "CBNZ" => {
            let reg = insn.operands.split(',').next().unwrap_or("?").trim();
            let name = resolve_reg(reg, reg_names);
            format!("else ({} == null)", name)
        }
        "TBZ" => {
            let parts: Vec<&str> = insn.operands.splitn(3, ',').collect();
            if parts.len() >= 2 {
                let reg = parts[0].trim();
                let name = resolve_reg(reg, reg_names);
                let bit = parts[1].trim().trim_start_matches('#');
                if bit == "0" {
                    format!("else ({}.initialized)", name)
                } else {
                    format!("else (bit{} of {} != 0)", bit, name)
                }
            } else {
                "else (bit != 0)".to_string()
            }
        }
        "TBNZ" => {
            let parts: Vec<&str> = insn.operands.splitn(3, ',').collect();
            if parts.len() >= 2 {
                let reg = parts[0].trim();
                let name = resolve_reg(reg, reg_names);
                let bit = parts[1].trim().trim_start_matches('#');
                if bit == "0" {
                    format!("else (!{}.initialized)", name)
                } else {
                    format!("else (bit{} of {} == 0)", bit, name)
                }
            } else {
                "else (bit == 0)".to_string()
            }
        }
        _ => {
            if let Some(ref cond) = insn.condition_code {
                let negated = negate_operator(cond);
                if let Some(cmp_ops) = last_cmp {
                    let parts: Vec<&str> = cmp_ops.splitn(2, ',').collect();
                    if parts.len() == 2 {
                        let lhs = parts[0].trim();
                        let rhs = parts[1].trim();
                        format!("else ({} {} {})", lhs, negated, rhs)
                    } else {
                        format!("else ({})", negated)
                    }
                } else {
                    format!("else ({})", negated)
                }
            } else {
                "else".to_string()
            }
        }
    }
}

fn negate_operator(op: &str) -> &str {
    match op {
        "==" => "!=",
        "!=" => "==",
        ">" => "<=",
        "<" => ">=",
        ">=" => "<",
        "<=" => ">",
        "> (unsigned)" => "<= (unsigned)",
        "< (unsigned)" => ">= (unsigned)",
        ">= (unsigned)" => "< (unsigned)",
        "<= (unsigned)" => "> (unsigned)",
        _ => op,
    }
}

fn build_branch_label(insn: &DisassembledInstruction, cond: &str, reg_names: Option<&HashMap<String, String>>) -> String {
    match insn.mnemonic.as_str() {
        "CBZ" => {
            let reg = insn.operands.split(',').next().unwrap_or("?").trim();
            let name = resolve_reg(reg, reg_names);
            format!("if ({} == null) goto 0x{:08X}", name, insn.branch_target.unwrap_or(0))
        }
        "CBNZ" => {
            let reg = insn.operands.split(',').next().unwrap_or("?").trim();
            let name = resolve_reg(reg, reg_names);
            format!("if ({} != null) goto 0x{:08X}", name, insn.branch_target.unwrap_or(0))
        }
        "TBZ" => {
            let parts: Vec<&str> = insn.operands.splitn(3, ',').collect();
            let reg = parts.first().map(|s| s.trim()).unwrap_or("?");
            let name = resolve_reg(reg, reg_names);
            let bit = parts.get(1).map(|s| s.trim().trim_start_matches('#')).unwrap_or("0");
            if bit == "0" {
                format!("if (!{}.initialized) goto 0x{:08X}", name, insn.branch_target.unwrap_or(0))
            } else {
                format!("if (bit{} of {} == 0) goto 0x{:08X}", bit, name, insn.branch_target.unwrap_or(0))
            }
        }
        "TBNZ" => {
            let parts: Vec<&str> = insn.operands.splitn(3, ',').collect();
            let reg = parts.first().map(|s| s.trim()).unwrap_or("?");
            let name = resolve_reg(reg, reg_names);
            let bit = parts.get(1).map(|s| s.trim().trim_start_matches('#')).unwrap_or("0");
            if bit == "0" {
                format!("if ({}.initialized) goto 0x{:08X}", name, insn.branch_target.unwrap_or(0))
            } else {
                format!("if (bit{} of {} != 0) goto 0x{:08X}", bit, name, insn.branch_target.unwrap_or(0))
            }
        }
        _ => {
            format!("if ({}) goto 0x{:08X}", cond, insn.branch_target.unwrap_or(0))
        }
    }
}

fn format_hex_bytes(bytes: &[u8]) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        hex.push_str(&format!("{:02X}", b));
    }
    hex
}

fn extract_adrp_page(insn: &DisassembledInstruction) -> Option<u64> {
    let operands = &insn.operands;
    if let Some(dollar_pos) = operands.find("$+") {
        let after = &operands[dollar_pos + 2..];
        let hex_str = after.trim().trim_start_matches("0x").trim_start_matches("0X");
        if let Ok(offset) = u64::from_str_radix(hex_str, 16) {
            return Some(insn.address.wrapping_add(offset));
        }
    }
    if let Some(dollar_pos) = operands.find("$-") {
        let after = &operands[dollar_pos + 2..];
        let hex_str = after.trim().trim_start_matches("0x").trim_start_matches("0X");
        if let Ok(offset) = u64::from_str_radix(hex_str, 16) {
            return Some(insn.address.wrapping_sub(offset));
        }
    }
    if let Some(hash_pos) = operands.find("#0x") {
        let after = &operands[hash_pos + 3..];
        let end = after.find(|c: char| !c.is_ascii_hexdigit()).unwrap_or(after.len());
        if let Ok(val) = u64::from_str_radix(&after[..end], 16) {
            return Some(val);
        }
    }
    None
}
