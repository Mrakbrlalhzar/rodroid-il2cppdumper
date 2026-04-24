use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, IntelFormatter, Mnemonic, OpKind};
use super::DisassembledInstruction;

pub fn disassemble_x86(
    bytes: &[u8],
    base_address: u64,
    max_instructions: usize,
    bitness: u32,
) -> Vec<DisassembledInstruction> {
    let mut result = Vec::with_capacity(max_instructions.min(512));
    let mut decoder = Decoder::with_ip(bitness, bytes, base_address, DecoderOptions::NONE);
    let mut formatter = IntelFormatter::new();

    formatter.options_mut().set_uppercase_mnemonics(true);
    formatter.options_mut().set_space_after_operand_separator(true);
    formatter.options_mut().set_hex_prefix("0x");
    formatter.options_mut().set_hex_suffix("");
    formatter.options_mut().set_branch_leading_zeros(false);

    let mut instruction = Instruction::default();
    let mut output = String::with_capacity(64);
    let mut count = 0;
    let mut seen_ret = false;

    while decoder.can_decode() && count < max_instructions {
        decoder.decode_out(&mut instruction);

        output.clear();
        formatter.format(&instruction, &mut output);

        let (mnemonic_str, operands) = split_mnemonic_operands(&output);

        let is_return = matches!(
            instruction.mnemonic(),
            Mnemonic::Ret | Mnemonic::Retf
        );

        let is_call = matches!(instruction.mnemonic(), Mnemonic::Call);

        let is_jcc = is_jcc_mnemonic(&instruction);

        let is_unconditional_branch = matches!(
            instruction.mnemonic(),
            Mnemonic::Jmp | Mnemonic::Ret | Mnemonic::Retf
        );

        let is_branch = is_unconditional_branch || is_call || is_jcc;

        let call_target = if is_call {
            extract_branch_target(&instruction)
        } else {
            None
        };

        let branch_target = if (is_jcc || is_unconditional_branch) && !is_return {
            extract_branch_target(&instruction)
        } else {
            None
        };

        let condition_code = if is_jcc {
            extract_x86_condition(&instruction)
        } else {
            None
        };

        let memory_offset = extract_memory_offset(&instruction);

        let raw_start = (instruction.ip() - base_address) as usize;
        let raw_end = (raw_start + instruction.len()).min(bytes.len());
        let raw_bytes = if raw_start < bytes.len() {
            bytes[raw_start..raw_end].to_vec()
        } else {
            vec![0; instruction.len()]
        };

        result.push(DisassembledInstruction {
            address: instruction.ip(),
            size: instruction.len(),
            raw_bytes,
            mnemonic: mnemonic_str,
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

        count += 1;

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
        let mnemonic = trimmed[..pos].to_string();
        let operands = trimmed[pos..].trim().to_string();
        (mnemonic, operands)
    } else {
        (trimmed.to_string(), String::new())
    }
}

fn is_jcc_mnemonic(instruction: &Instruction) -> bool {
    matches!(
        instruction.mnemonic(),
        Mnemonic::Ja | Mnemonic::Jae | Mnemonic::Jb | Mnemonic::Jbe
        | Mnemonic::Je | Mnemonic::Jne | Mnemonic::Jg | Mnemonic::Jge
        | Mnemonic::Jl | Mnemonic::Jle | Mnemonic::Jo | Mnemonic::Jno
        | Mnemonic::Jp | Mnemonic::Jnp | Mnemonic::Js | Mnemonic::Jns
        | Mnemonic::Jecxz | Mnemonic::Jrcxz
        | Mnemonic::Loop | Mnemonic::Loope | Mnemonic::Loopne
    )
}

fn extract_branch_target(instruction: &Instruction) -> Option<u64> {
    if instruction.op_count() >= 1 {
        match instruction.op0_kind() {
            OpKind::NearBranch16 => Some(instruction.near_branch16() as u64),
            OpKind::NearBranch32 => Some(instruction.near_branch32() as u64),
            OpKind::NearBranch64 => Some(instruction.near_branch64()),
            OpKind::FarBranch16 => Some(instruction.far_branch16() as u64),
            OpKind::FarBranch32 => Some(instruction.far_branch32() as u64),
            _ => None,
        }
    } else {
        None
    }
}

fn extract_x86_condition(instruction: &Instruction) -> Option<String> {
    let cond = match instruction.mnemonic() {
        Mnemonic::Je => "==",
        Mnemonic::Jne => "!=",
        Mnemonic::Jg => ">",
        Mnemonic::Jge => ">=",
        Mnemonic::Jl => "<",
        Mnemonic::Jle => "<=",
        Mnemonic::Ja => "> (unsigned)",
        Mnemonic::Jae => ">= (unsigned)",
        Mnemonic::Jb => "< (unsigned)",
        Mnemonic::Jbe => "<= (unsigned)",
        Mnemonic::Jo => "overflow",
        Mnemonic::Jno => "!overflow",
        Mnemonic::Js => "< 0 (sign)",
        Mnemonic::Jns => ">= 0 (sign)",
        Mnemonic::Jp => "parity",
        Mnemonic::Jnp => "!parity",
        Mnemonic::Jecxz | Mnemonic::Jrcxz => "counter == 0",
        Mnemonic::Loop => "counter != 0",
        Mnemonic::Loope => "counter != 0 && ==",
        Mnemonic::Loopne => "counter != 0 && !=",
        _ => return None,
    };
    Some(cond.to_string())
}

fn extract_memory_offset(instruction: &Instruction) -> Option<i64> {
    for i in 0..instruction.op_count() {
        let kind = match i {
            0 => instruction.op0_kind(),
            1 => instruction.op1_kind(),
            2 => instruction.op2_kind(),
            3 => instruction.op3_kind(),
            _ => continue,
        };

        if kind == OpKind::Memory {
            let disp = instruction.memory_displacement64() as i64;
            if disp != 0 {
                return Some(disp);
            }
        }
    }
    None
}
