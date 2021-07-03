use crate::arch;

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
     value: arch::Word
}

impl Instruction {
    pub fn op_code(&self) -> OpCode {
        to_opcode(self.value.bytes[4].read())
    }

    pub fn modification(self) -> u8 {
        self.value.bytes[3].read()
    }

    pub fn index_specification(&self) -> u8 {
        self.value.bytes[2].read()
    }

    pub fn field_modifier(&self) -> (u8, u8) {
        let val = self.value.bytes[3].read();
        (val / 8, val % 8)
    }

    pub fn address(&self) -> arch::HalfWord {
        arch::HalfWord{
            is_positive: self.value.is_positive,
            bytes: [self.value.bytes[0], self.value.bytes[1]]
        }
    }

    pub fn new(op_code: OpCode, modification: u8, index_specification: u8, address: arch::HalfWord) -> Instruction{
        Instruction {
            value: arch::Word::from_values(address.is_positive, address.bytes[0].read(), address.bytes[1].read(), index_specification, modification, op_code as u8)
        }
    }

    pub fn from_word(word: arch::Word) -> Instruction {
        Instruction {
            value: word
        }
    }

    //do not include JBUS because that is a no-op for us
    pub fn is_jump(self) -> bool{
        match self.op_code() {
            OpCode::Jump => true,
            OpCode::JRED => true,
            OpCode::JumpA => true,
            OpCode::JumpX => true,
            OpCode::JumpI1 => true,
            OpCode::JumpI2 => true,
            OpCode::JumpI3 => true,
            OpCode::JumpI4 => true,
            OpCode::JumpI5 => true,
            OpCode::JumpI6 => true,
            _ => false
        }
    }

}

fn to_opcode(val: u8) -> OpCode {
    match val {
        0 => OpCode::NOP,
        1 => OpCode::ADD,
        2 => OpCode::SUB,
        3 => OpCode::MUL,
        4 => OpCode::DIV,
        5 => OpCode::HaltNumChar,
        6 => OpCode::Shift,
        7 => OpCode::MOVE,
        8 => OpCode::LDA,
        9 => OpCode::LD1,
        10 => OpCode::LD2,
        11 => OpCode::LD3,
        12 => OpCode::LD4,
        13 => OpCode::LD5,
        14 => OpCode::LD6,
        15 => OpCode::LDX,
        16 => OpCode::LDAN,
        17 => OpCode::LD1N,
        18 => OpCode::LD2N,
        19 => OpCode::LD3N,
        20 => OpCode::LD4N,
        21 => OpCode::LD5N,
        22 => OpCode::LD6N,
        23 => OpCode::LDXN,
        24 => OpCode::STA,
        25 => OpCode::ST1,
        26 => OpCode::ST2,
        27 => OpCode::ST3,
        28 => OpCode::ST4,
        29 => OpCode::ST5,
        30 => OpCode::ST6,
        31 => OpCode::STX,
        32 => OpCode::STJ,
        33 => OpCode::STZ,
        34 => OpCode::JBUS,
        35 => OpCode::IOC,
        36 => OpCode::IN,
        37 => OpCode::OUT,
        38 => OpCode::JRED,
        39 => OpCode::Jump,
        40 => OpCode::JumpA,
        41 => OpCode::JumpI1,
        42 => OpCode::JumpI2,
        43 => OpCode::JumpI3,
        44 => OpCode::JumpI4,
        45 => OpCode::JumpI5,
        46 => OpCode::JumpI6,
        47 => OpCode::JumpX,
        48 => OpCode::AddressTransferA,
        49 => OpCode::AddressTransferI1,
        50 => OpCode::AddressTransferI2,
        51 => OpCode::AddressTransferI3,
        52 => OpCode::AddressTransferI4,
        53 => OpCode::AddressTransferI5,
        54 => OpCode::AddressTransferI6,
        55 => OpCode::AddressTransferX,
        56 => OpCode::CMPA,
        57 => OpCode::CMP1,
        58 => OpCode::CMP2,
        59 => OpCode::CMP3,
        60 => OpCode::CMP4,
        61 => OpCode::CMP5,
        62 => OpCode::CMP6,
        63 => OpCode::CMPX,
        _ => panic!("Invalid OpCode"),
    }
}


#[derive(Debug)]
pub enum OpCode {
    NOP = 0,
    ADD = 1,
    SUB = 2,
    MUL = 3,
    DIV = 4,
    HaltNumChar = 5,
    Shift = 6,
    MOVE = 7,
    LDA = 8,
    LD1 = 9,
    LD2 = 10,
    LD3 = 11,
    LD4 = 12,
    LD5 = 13,
    LD6 = 14,
    LDX = 15,
    LDAN = 16,
    LD1N = 17,
    LD2N = 18,
    LD3N = 19,
    LD4N = 20,
    LD5N = 21,
    LD6N = 22,
    LDXN = 23,
    STA = 24,
    ST1 = 25,
    ST2 = 26,
    ST3 = 27,
    ST4 = 28,
    ST5 = 29,
    ST6 = 30,
    STX = 31,
    STJ = 32,
    STZ = 33,
    JBUS = 34,
    IOC = 35,
    IN = 36,
    OUT = 37,
    JRED = 38,
    Jump = 39,
    JumpA = 40,
    JumpI1 = 41,
    JumpI2 = 42,
    JumpI3 = 43,
    JumpI4 = 44,
    JumpI5 = 45,
    JumpI6 = 46,
    JumpX = 47,
    AddressTransferA = 48,
    AddressTransferI1 = 49,
    AddressTransferI2 = 50,
    AddressTransferI3 = 51,
    AddressTransferI4 = 52,
    AddressTransferI5 = 53,
    AddressTransferI6 = 54,
    AddressTransferX = 55,
    CMPA = 56,
    CMP1 = 57,
    CMP2 = 58,
    CMP3 = 59,
    CMP4 = 60,
    CMP5 = 61,
    CMP6 = 62,
    CMPX = 63,
}

pub fn str_to_opcode(line: &str) -> OpCode {
    match line {
        "HLT" | "NUM" | "CHAR" => OpCode::HaltNumChar,
        "ADD" => OpCode::ADD,
        "SUB" => OpCode::SUB,
        "MUL" => OpCode::MUL,
        "DIV" => OpCode::DIV,
        "SLA" | "SLAX" | "SRA" | "SRAX" | "SLC" | "SRC" => OpCode::Shift,
        "MOVE" => OpCode::MOVE,
        "LDA" => OpCode::LDA,
        "LD1" => OpCode::LD1,
        "LD2" => OpCode::LD2,
        "LD3" => OpCode::LD3,
        "LD4" => OpCode::LD4,
        "LD5" => OpCode::LD5,
        "LD6" => OpCode::LD6,
        "LDX" => OpCode::LDX,
        "LDAN" => OpCode::LDAN,
        "LD1N" => OpCode::LD1N,
        "LD2N" => OpCode::LD2N,
        "LD3N" => OpCode::LD3N,
        "LD4N" => OpCode::LD4N,
        "LD5N" => OpCode::LD5N,
        "LD6N" => OpCode::LD6N,
        "LDXN" => OpCode::LDXN,
        "STA" => OpCode::STA,
        "ST1" => OpCode::ST1,
        "ST2" => OpCode::ST2,
        "ST3" => OpCode::ST3,
        "ST4" => OpCode::ST4,
        "ST5" => OpCode::ST5,
        "ST6" => OpCode::ST6,
        "STX" => OpCode::STX,
        "STJ" => OpCode::STJ,
        "STZ" => OpCode::STZ,
        "JBUS" => OpCode::JBUS,
        "IOC" => OpCode::IOC,
        "IN" => OpCode::IN,
        "OUT" => OpCode::OUT,
        "JRED" => OpCode::JRED,
        "JMP" | "JSJ" | "JOV" | "JNOV" | "JL" | "JE" | "JG" | "JGE" | "JNE" | "JLE" => OpCode::Jump,
        "JAN" | "JAZ" | "JAP" | "JANN" | "JANZ" | "JANP" => OpCode::JumpA,
        "J1N" | "J1Z" | "J1P" | "J1NN" | "J1NZ" | "J1NP" => OpCode::JumpI1,
        "J2N" | "J2Z" | "J2P" | "J2NN" | "J2NZ" | "J2NP" => OpCode::JumpI2,
        "J3N" | "J3Z" | "J3P" | "J3NN" | "J3NZ" | "J3NP" => OpCode::JumpI3,
        "J4N" | "J4Z" | "J4P" | "J4NN" | "J4NZ" | "J4NP" => OpCode::JumpI4,
        "J5N" | "J5Z" | "J5P" | "J5NN" | "J5NZ" | "J5NP" => OpCode::JumpI5,
        "J6N" | "J6Z" | "J6P" | "J6NN" | "J6NZ" | "J6NP" => OpCode::JumpI6,
        "JXN" | "JXZ" | "JXP" | "JXNN" | "JXNZ" | "JXNP" => OpCode::JumpX,
        "INCA" | "DECA" | "ENTA" | "ENNA" => OpCode::AddressTransferA,
        "INC1" | "DEC1" | "ENT1" | "ENN1" => OpCode::AddressTransferI1,
        "INC2" | "DEC2" | "ENT2" | "ENN2" => OpCode::AddressTransferI2,
        "INC3" | "DEC3" | "ENT3" | "ENN3" => OpCode::AddressTransferI3,
        "INC4" | "DEC4" | "ENT4" | "ENN4" => OpCode::AddressTransferI4,
        "INC5" | "DEC5" | "ENT5" | "ENN5" => OpCode::AddressTransferI5,
        "INC6" | "DEC6" | "ENT6" | "ENN6" => OpCode::AddressTransferI6,
        "INCX" | "DECX" | "ENTX" | "ENNX" => OpCode::AddressTransferX,
        "CMPA" => OpCode::CMPA,
        "CMP1" => OpCode::CMP1,
        "CMP2" => OpCode::CMP2,
        "CMP3" => OpCode::CMP3,
        "CMP4" => OpCode::CMP4,
        "CMP5" => OpCode::CMP5,
        "CMP6" => OpCode::CMP6,
        "CMPX" => OpCode::CMPX,
        _ => OpCode::NOP
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jbus_is_not_jump() {
        let instruction = Instruction::new(OpCode::JBUS, 5, 0, arch::HalfWord::from_value(2000));
        assert_eq!(instruction.is_jump(), false);
    }
}
