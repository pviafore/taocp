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

    pub fn to_string(&self) -> String {
        format!("{} {},{}({})", to_opcode_str(self.op_code() as u8, self.modification()), self.address().read(), self.index_specification(), self.modification())
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

fn to_opcode_str(val: u8, modifier: u8) -> &'static str {
    match (val, modifier) {
        (0, _) => "NOP",
        (1, _) => "ADD",
        (2, _) => "SUB",
        (3, _) => "MUL",
        (4, _) => "DIV",
        (5, 0) => "HALT",
        (5, 1) => "NUM",
        (5, 2) => "CHAR",
        (6, 0) => "SLA",
        (6, 1) => "SRA",
        (6, 2) => "SLAX",
        (6, 3) => "SLAX",
        (6, 4) => "SLC",
        (6, 5) => "SRC",
        (7, _) => "MOVE",
        (8, _) => "LDA",
        (9, _) => "LD1",
        (10, _) => "LD2",
        (11, _) => "LD3",
        (12, _) => "LD4",
        (13, _) => "LD5",
        (14, _) => "LD6",
        (15, _) => "LDX",
        (16, _) => "LDAN",
        (17, _) => "LD1N",
        (18, _) => "LD2N",
        (19, _) => "LD3N",
        (20, _) => "LD4N",
        (21, _) => "LD5N",
        (22, _) => "LD6N",
        (23, _) => "LDXN",
        (24, _) => "STA",
        (25, _) => "ST1",
        (26, _) => "ST2",
        (27, _) => "ST3",
        (28, _) => "ST4",
        (29, _) => "ST5",
        (30, _) => "ST6",
        (31, _) => "STX",
        (32, _) => "STJ",
        (33, _) => "STZ",
        (34, _) => "JBUS",
        (35, _) => "IOC",
        (36, _) => "IN",
        (37, _) => "OUT",
        (38, _) => "JRED",
        (39, 0) => "JMP",
        (39, 1) => "JSJ",
        (39, 2) => "JOV",
        (39, 3) => "JNOV",
        (39, 4) => "JL",
        (39, 5) => "JE",
        (39, 6) => "JG",
        (39, 7) => "JGE",
        (39, 8) => "JNE",
        (39, 9) => "JLE",
        (40, 0) => "JAN",
        (40, 1) => "JAZ",
        (40, 2) => "JAP",
        (40, 3) => "JANN",
        (40, 4) => "JANZ",
        (40, 5) => "JANP",
        (41, 0) => "J1N",
        (41, 1) => "J1Z",
        (41, 2) => "J1P",
        (41, 3) => "J1NN",
        (41, 4) => "J1NZ",
        (41, 5) => "J1NP",
        (42, 0) => "J2N",
        (42, 1) => "J2Z",
        (42, 2) => "J2P",
        (42, 3) => "J2NN",
        (42, 4) => "J2NZ",
        (42, 5) => "J2NP",
        (43, 0) => "J3N",
        (43, 1) => "J3Z",
        (43, 2) => "J3P",
        (43, 3) => "J3NN",
        (43, 4) => "J3NZ",
        (43, 5) => "J3NP",
        (44, 0) => "J4N",
        (44, 1) => "J4Z",
        (44, 2) => "J4P",
        (44, 3) => "J4NN",
        (44, 4) => "J4NZ",
        (44, 5) => "J4NP",
        (45, 0) => "J5N",
        (45, 1) => "J5Z",
        (45, 2) => "J5P",
        (45, 3) => "J5NN",
        (45, 4) => "J5NZ",
        (45, 5) => "J5NP",
        (46, 0) => "J6N",
        (46, 1) => "J6Z",
        (46, 2) => "J6P",
        (46, 3) => "J6NN",
        (46, 4) => "J6NZ",
        (46, 5) => "J6NP",
        (47, 0) => "JXN",
        (47, 1) => "JXZ",
        (47, 2) => "JXP",
        (47, 3) => "JXNN",
        (47, 4) => "JXNZ",
        (47, 5) => "JXNP",
        (48, 0) => "INCA",
        (48, 1) => "DECA",
        (48, 2) => "ENTA",
        (48, 3) => "ENNA",
        (49, 0) => "INC1",
        (49, 1) => "DEC1",
        (49, 2) => "ENT1",
        (49, 3) => "ENN1",
        (50, 0) => "INC2",
        (50, 1) => "DEC2",
        (50, 2) => "ENT2",
        (50, 3) => "ENN2",
        (51, 0) => "INC3",
        (51, 1) => "DEC3",
        (51, 2) => "ENT3",
        (51, 3) => "ENN3",
        (52, 0) => "INC4",
        (52, 1) => "DEC4",
        (52, 2) => "ENT4",
        (52, 3) => "ENN4",
        (53, 0) => "INC5",
        (53, 1) => "DEC5",
        (53, 2) => "ENT5",
        (53, 3) => "ENN5",
        (54, 0) => "INC6",
        (54, 1) => "DEC6",
        (54, 2) => "ENT6",
        (54, 3) => "ENN6",
        (55, 0) => "INCX",
        (55, 1) => "DECX",
        (55, 2) => "ENTX",
        (55, 3) => "ENNX",
        (56, _) => "CMPA",
        (57, _) => "CMP1",
        (58, _) => "CMP2",
        (59, _) => "CMP3",
        (60, _) => "CMP4",
        (61, _) => "CMP5",
        (62, _) => "CMP6",
        (63, _) => "CMPX",
        _ => "INVALID"
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
