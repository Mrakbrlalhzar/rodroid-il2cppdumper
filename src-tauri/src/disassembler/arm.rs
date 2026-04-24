use yaxpeax_arch::{Decoder, LengthedInstruction, U8Reader};
use yaxpeax_arm::armv8::a64::{InstDecoder as A64Decoder, Instruction as A64Instruction, Opcode as A64Opcode, Operand as A64Operand};
use yaxpeax_arm::armv7::{InstDecoder as A32Decoder, Instruction as A32Instruction};
use super::DisassembledInstruction;

pub fn disassemble_arm64(
    bytes: &[u8],
    base_address: u64,
    max_instructions: usize,
) -> Vec<DisassembledInstruction> {
    let mut result = Vec::with_capacity(max_instructions.min(512));
    let decoder = A64Decoder::default();
    let mut reader = U8Reader::new(bytes);
    let mut offset: u64 = 0;
    let mut seen_ret = false;

    for _ in 0..max_instructions {
        if offset as usize >= bytes.len() {
            break;
        }

        let insn: A64Instruction = match decoder.decode(&mut reader) {
            Ok(i) => i,
            Err(_) => break,
        };

        let address = base_address + offset;
        let size = insn.len().to_const() as usize;

        let raw_start = offset as usize;
        let raw_end = (raw_start + size).min(bytes.len());
        let raw_bytes = bytes[raw_start..raw_end].to_vec();

        let full_text = format!("{}", insn);
        let (mnemonic, operands) = split_mnemonic_operands(&full_text);

        let is_return = matches!(insn.opcode, A64Opcode::RET);

        let is_call = matches!(insn.opcode, A64Opcode::BL | A64Opcode::BLR);

        let is_unconditional_branch = matches!(
            insn.opcode,
            A64Opcode::B | A64Opcode::BR | A64Opcode::RET
        );

        let is_conditional_branch = matches!(
            insn.opcode,
            A64Opcode::Bcc(_) | A64Opcode::CBZ | A64Opcode::CBNZ
            | A64Opcode::TBZ | A64Opcode::TBNZ
        );

        let is_branch = is_unconditional_branch || is_call || is_conditional_branch;

        let call_target = if is_call {
            extract_arm64_branch_target(&insn, address)
        } else {
            None
        };

        let branch_target = if is_branch && !is_call && !is_return {
            extract_arm64_branch_target(&insn, address)
        } else {
            None
        };

        let condition_code = extract_arm64_condition(&insn, &operands);

        let memory_offset = extract_arm64_memory_offset(&insn);

        result.push(DisassembledInstruction {
            address,
            size,
            raw_bytes,
            mnemonic,
            operands,
            is_call,
            is_return,
            is_branch,
            is_unconditional_branch,
            call_target,
            branch_target,
            condition_code,
            memory_offset,
        });

        offset += size as u64;

        if is_return {
            if seen_ret {
                break;
            }
            seen_ret = true;
        }
    }

    result
}

pub fn disassemble_arm32(
    bytes: &[u8],
    base_address: u64,
    max_instructions: usize,
) -> Vec<DisassembledInstruction> {
    let mut result = Vec::with_capacity(max_instructions.min(512));
    let decoder = A32Decoder::default();
    let mut reader = U8Reader::new(bytes);
    let mut offset: u64 = 0;
    let mut seen_ret = false;

    for _ in 0..max_instructions {
        if offset as usize >= bytes.len() {
            break;
        }

        let insn: A32Instruction = match decoder.decode(&mut reader) {
            Ok(i) => i,
            Err(_) => break,
        };

        let address = base_address + offset;
        let size = insn.len().to_const() as usize;

        let raw_start = offset as usize;
        let raw_end = (raw_start + size).min(bytes.len());
        let raw_bytes = bytes[raw_start..raw_end].to_vec();

        let full_text = format!("{}", insn);
        let (mnemonic, operands) = split_mnemonic_operands(&full_text);

        let is_return = full_text.contains("pop") && full_text.contains("pc")
            || full_text.starts_with("bx lr");

        let is_call = full_text.starts_with("bl ") || full_text.starts_with("blx ");

        let is_unconditional_branch = mnemonic == "B" || mnemonic == "BX" || is_return;

        let is_branch = is_unconditional_branch || is_call
            || mnemonic.starts_with("B") && mnemonic != "BIC" && mnemonic != "BFC" && mnemonic != "BFI";

        let call_target = if is_call {
            extract_arm32_branch_target(&full_text, address)
        } else {
            None
        };

        let branch_target = if is_branch && !is_call && !is_return {
            extract_arm32_branch_target(&full_text, address)
        } else {
            None
        };

        let condition_code = extract_arm32_condition(&mnemonic);

        let memory_offset = extract_arm32_memory_offset(&full_text);

        result.push(DisassembledInstruction {
            address,
            size,
            raw_bytes,
            mnemonic,
            operands,
            is_call,
            is_return,
            is_branch,
            is_unconditional_branch,
            call_target,
            branch_target,
            condition_code,
            memory_offset,
        });

        offset += size as u64;

        if is_return {
            if seen_ret {
                break;
            }
            seen_ret = true;
        }
    }

    result
}

fn split_mnemonic_operands(text: &str) -> (String, String) {
    let trimmed = text.trim();
    if let Some(pos) = trimmed.find(' ') {
        let mnemonic = trimmed[..pos].to_uppercase();
        let operands = trimmed[pos..].trim().to_string();
        (mnemonic, operands)
    } else {
        (trimmed.to_uppercase(), String::new())
    }
}

fn extract_arm64_branch_target(insn: &A64Instruction, current_address: u64) -> Option<u64> {
    for i in 0..4 {
        match insn.operands[i] {
            A64Operand::PCOffset(offset) => {
                return Some((current_address as i64 + offset as i64) as u64);
            }
            A64Operand::Imm64(val) => {
                return Some(val);
            }
            A64Operand::Imm16(val) => {
                return Some(val as u64);
            }
            A64Operand::Nothing => break,
            _ => {}
        }
    }
    None
}

fn extract_arm64_condition(insn: &A64Instruction, operands: &str) -> Option<String> {
    match insn.opcode {
        A64Opcode::Bcc(cond) => {
            let cond_str = match cond {
                0 => "==",       // EQ
                1 => "!=",       // NE
                2 => ">= (unsigned)", // CS/HS
                3 => "< (unsigned)",  // CC/LO
                4 => "< 0",     // MI (negative)
                5 => ">= 0",    // PL (positive/zero)
                6 => "overflow", // VS
                7 => "!overflow", // VC
                8 => "> (unsigned)", // HI
                9 => "<= (unsigned)", // LS
                10 => ">=",      // GE
                11 => "<",       // LT
                12 => ">",       // GT
                13 => "<=",      // LE
                14 => "always",  // AL
                _ => "?",
            };
            Some(cond_str.to_string())
        }
        A64Opcode::CBZ => {
            let reg = operands.split(',').next().unwrap_or("?").trim();
            Some(format!("{} == null", reg))
        }
        A64Opcode::CBNZ => {
            let reg = operands.split(',').next().unwrap_or("?").trim();
            Some(format!("{} != null", reg))
        }
        A64Opcode::TBZ => {
            let parts: Vec<&str> = operands.splitn(3, ',').collect();
            if parts.len() >= 2 {
                let reg = parts[0].trim();
                let bit = parts[1].trim().trim_start_matches('#');
                Some(format!("bit{} of {} == 0", bit, reg))
            } else {
                Some("bit == 0".to_string())
            }
        }
        A64Opcode::TBNZ => {
            let parts: Vec<&str> = operands.splitn(3, ',').collect();
            if parts.len() >= 2 {
                let reg = parts[0].trim();
                let bit = parts[1].trim().trim_start_matches('#');
                Some(format!("bit{} of {} != 0", bit, reg))
            } else {
                Some("bit != 0".to_string())
            }
        }
        _ => None,
    }
}

fn extract_arm32_condition(mnemonic: &str) -> Option<String> {
    if mnemonic.len() < 2 { return None; }
    if !mnemonic.starts_with('B') { return None; }
    if mnemonic == "B" || mnemonic == "BL" || mnemonic == "BX" || mnemonic == "BLX"
        || mnemonic == "BIC" || mnemonic == "BFC" || mnemonic == "BFI" {
        return None;
    }

    let suffix = &mnemonic[1..];
    let cond = match suffix {
        "EQ" => "==",
        "NE" => "!=",
        "CS" | "HS" => ">= (unsigned)",
        "CC" | "LO" => "< (unsigned)",
        "MI" => "< 0",
        "PL" => ">= 0",
        "VS" => "overflow",
        "VC" => "!overflow",
        "HI" => "> (unsigned)",
        "LS" => "<= (unsigned)",
        "GE" => ">=",
        "LT" => "<",
        "GT" => ">",
        "LE" => "<=",
        _ => return None,
    };
    Some(cond.to_string())
}

fn extract_arm64_memory_offset(insn: &A64Instruction) -> Option<i64> {
    for i in 0..4 {
        match insn.operands[i] {
            A64Operand::RegRegOffset(..) => {}
            A64Operand::RegPreIndex(_, offset, _) => {
                return Some(offset as i64);
            }
            A64Operand::RegPostIndex(_, offset) => {
                return Some(offset as i64);
            }
            A64Operand::RegPostIndexReg(_, _) => {}
            A64Operand::Nothing => break,
            _ => {}
        }
    }

    let text = format!("{}", insn);
    if let Some(bracket_start) = text.find('[') {
        if let Some(hash_pos) = text[bracket_start..].find('#') {
            let after_hash = &text[bracket_start + hash_pos + 1..];
            let end = after_hash.find(']')
                .or_else(|| after_hash.find(','))
                .unwrap_or(after_hash.len());
            let num_str = after_hash[..end].trim().trim_end_matches('!');

            if let Some(stripped) = num_str.strip_prefix("0x").or_else(|| num_str.strip_prefix("0X")) {
                if let Ok(val) = i64::from_str_radix(stripped, 16) {
                    return Some(val);
                }
            } else if let Some(stripped) = num_str.strip_prefix("-0x").or_else(|| num_str.strip_prefix("-0X")) {
                if let Ok(val) = i64::from_str_radix(stripped, 16) {
                    return Some(-val);
                }
            } else if let Ok(val) = num_str.parse::<i64>() {
                return Some(val);
            }
        }
    }

    None
}

fn extract_arm32_branch_target(text: &str, current_address: u64) -> Option<u64> {
    let parts: Vec<&str> = text.split_whitespace().collect();
    if parts.len() >= 2 {
        let target_str = parts.last()?;
        let target_str = target_str.trim_start_matches("$+");
        let target_str = target_str.trim_start_matches("0x");

        if let Ok(offset) = i64::from_str_radix(target_str, 16) {
            return Some((current_address as i64 + offset) as u64);
        }

        if let Ok(offset) = target_str.parse::<i64>() {
            return Some((current_address as i64 + offset) as u64);
        }
    }
    None
}

fn extract_arm32_memory_offset(text: &str) -> Option<i64> {
    if let Some(bracket_start) = text.find('[') {
        if let Some(hash_pos) = text[bracket_start..].find('#') {
            let after_hash = &text[bracket_start + hash_pos + 1..];
            let end = after_hash.find(']')
                .or_else(|| after_hash.find(','))
                .unwrap_or(after_hash.len());
            let num_str = after_hash[..end].trim();

            if let Some(stripped) = num_str.strip_prefix("0x") {
                if let Ok(val) = i64::from_str_radix(stripped, 16) {
                    return Some(val);
                }
            } else if let Some(stripped) = num_str.strip_prefix("-0x") {
                if let Ok(val) = i64::from_str_radix(stripped, 16) {
                    return Some(-val);
                }
            } else if let Ok(val) = num_str.parse::<i64>() {
                return Some(val);
            }
        }
    }
    None
}
