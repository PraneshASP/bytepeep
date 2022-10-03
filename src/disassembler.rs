use crate::types::*;
use crate::evm::*;

fn match_push_n(opcode: Opcode) -> u8 {
    match opcode {
        Opcode::Push1 => 1,
        Opcode::Push2 => 2,
        Opcode::Push3 => 3,
        Opcode::Push4 => 4,
        Opcode::Push5 => 5,
        Opcode::Push6 => 6,
        Opcode::Push7 => 7,
        Opcode::Push8 => 8,
        Opcode::Push9 => 9,
        Opcode::Push10 => 10,
        Opcode::Push11 => 11,
        Opcode::Push12 => 12,
        Opcode::Push13 => 13,
        Opcode::Push14 => 14,
        Opcode::Push15 => 15,
        Opcode::Push16 => 16,
        Opcode::Push17 => 17,
        Opcode::Push18 => 18,
        Opcode::Push19 => 19,
        Opcode::Push20 => 20,
        Opcode::Push21 => 21,
        Opcode::Push22 => 22,
        Opcode::Push23 => 23,
        Opcode::Push24 => 24,
        Opcode::Push25 => 25,
        Opcode::Push26 => 26,
        Opcode::Push27 => 27,
        Opcode::Push28 => 28,
        Opcode::Push29 => 29,
        Opcode::Push30 => 30,
        Opcode::Push31 => 31,
        Opcode::Push32 => 32,
        _              => 0
    }
}

fn print_output(bytecode: &Bytecode) {
    for byte in bytecode {
        if byte.kind == ByteKind::Opcode {
            print!("\n{} {} ", byte.pc, byte.opcode.unwrap().op_string());
        }
        if byte.kind == ByteKind::PushData {
            print!("{}", byte.pushdata.as_ref().unwrap());
        }
    }
}

pub fn disassemble(byte_string: &String) -> Bytecode {
    let mut pc: u32 = 0;
    let mut bytes_to_push: u8 = 0;
    let trimmed_byte_string: &str;

    if byte_string.starts_with(&String::from("0x")) {
        trimmed_byte_string = &byte_string[2..];
    } else {
        trimmed_byte_string = byte_string;
    }

    let bytecode: Bytecode = (0..trimmed_byte_string.len()).step_by(2).map(|byte| {
        if bytes_to_push > 0 {
            bytes_to_push -= 1;
            pc += 1;

            return ByteData {
                pc: pc - 1,
                opcode: None,
                pushdata: Some(String::from(&trimmed_byte_string[byte..byte + 2])),
                kind: ByteKind::PushData
            }
        }
    
        let opcode = Opcode::new(&trimmed_byte_string[byte..byte + 2]);
        bytes_to_push = match_push_n(opcode);

        pc += 1;
        ByteData {
            pc: pc - 1,
            opcode: Some(opcode),
            pushdata: None,
            kind: ByteKind::Opcode
        }
    }).collect();

    print_output(&bytecode);
    bytecode
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble() {
        let byte_string = String::from("0x60806054");
        let disassembled_bytes: Bytecode = vec![
            ByteData { pc: 0, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 1, opcode: None, pushdata: Some(String::from("80")), kind: ByteKind::PushData }, 
            ByteData { pc: 2, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 3, opcode: None, pushdata: Some(String::from("54")), kind: ByteKind::PushData }
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string));
    }

    #[test]
    fn test_disassemble_no_0x() {
        let byte_string = String::from("60806054");
        let disassembled_bytes: Bytecode = vec![
            ByteData { pc: 0, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 1, opcode: None, pushdata: Some(String::from("80")), kind: ByteKind::PushData }, 
            ByteData { pc: 2, opcode: Some(Opcode::Push1), pushdata: None, kind: ByteKind::Opcode }, 
            ByteData { pc: 3, opcode: None, pushdata: Some(String::from("54")), kind: ByteKind::PushData }
        ];
        assert_eq!(disassembled_bytes, disassemble(&byte_string));
    }
}