use crate::arch;
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
        8 => OpCode::LDA,
        _ => panic!("Invalid OpCode"),
    }
}
pub enum OpCode {
    LDA = 8,
}
