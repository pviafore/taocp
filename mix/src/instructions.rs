use crate::arch;

#[derive(Clone, Copy)]
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
}

fn to_opcode(val: u8) -> OpCode {
    match val {
        1 => OpCode::ADD,
        2 => OpCode::SUB,
        3 => OpCode::MUL,
        4 => OpCode::DIV,
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
        19=> OpCode::LD3N,
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
        31=> OpCode::STX,
        32 => OpCode::STJ,
        33 => OpCode::STJ,
        48 => OpCode::AddressTransferA,
        49 => OpCode::AddressTransferI1,
        50 => OpCode::AddressTransferI2,
        51 => OpCode::AddressTransferI3,
        52 => OpCode::AddressTransferI4,
        53 => OpCode::AddressTransferI5,
        54 => OpCode::AddressTransferI6,
        55 => OpCode::AddressTransferX,
        _ => panic!("Invalid OpCode"),
    }
}
pub enum OpCode {
    ADD = 1,
    SUB = 2,
    MUL = 3,
    DIV = 4,
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
    AddressTransferA = 48,
    AddressTransferI1 = 49,
    AddressTransferI2 = 50,
    AddressTransferI3 = 51,
    AddressTransferI4 = 52,
    AddressTransferI5 = 53,
    AddressTransferI6 = 54,
    AddressTransferX = 55 
}
