use crate::arch;
use crate::instructions;

use instructions::Instruction;
struct Registers {
    a: arch::Word,
    x: arch::Word,
    i1: arch::HalfWord,
    i2: arch::HalfWord,
    i3: arch::HalfWord,
    i4: arch::HalfWord,
    i5: arch::HalfWord,
    i6: arch::HalfWord,
    j: arch::HalfWord
}

enum ComparisonIndicator {
    LESS,
    EQUAL,
    GREATER
}
struct Computer {
    registers: Registers,
    overflow: bool,
    comparison: ComparisonIndicator,
    memory: Vec<arch::Word>,
    io: Vec<arch::Word>
}

impl Computer {
    pub fn run_command(&mut self, instruction: Instruction){
        let op = match instruction.op_code() {
            instructions::OpCode::LDA => Computer::loada
        };
        op(self, instruction)
    }

    pub fn new() -> Computer{
        use std::iter::FromIterator;
        Computer { 
            registers: Registers {
                a: arch::Word::new(),
                x: arch::Word::new(),
                i1: arch::HalfWord::new(),
                i2: arch::HalfWord::new(),
                i3: arch::HalfWord::new(),
                i4: arch::HalfWord::new(),
                i5: arch::HalfWord::new(),
                i6: arch::HalfWord::new(),
                j: arch::HalfWord::new()
            },
            overflow: false,
            comparison: ComparisonIndicator::EQUAL,
            memory: Vec::from_iter((0..4000).map(|_| arch::Word::new())),
            io: Vec::from_iter((0..20).map(|_| arch::Word::new()))
        }
    }

    fn loada(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        if address < 0 || address >= 4000 {
            panic!("Address cannot be negative or out of range")
        }
        let (l,r) = instruction.field_modifier();
        self.registers.a = self.memory[address as usize].read_partial_as_word(l,r);
    }

    fn get_offset(&mut self, val: u8) -> i16 {
        match val {
            0 => 0,
            1 => self.registers.i1.read(),
            2 => self.registers.i2.read(),
            3 => self.registers.i3.read(),
            4 => self.registers.i4.read(),
            5 => self.registers.i5.read(),
            6 => self.registers.i6.read(),
            _ => panic!("Unknown index specifier {:?}", val)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loada() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LDA, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 1, 16, 3, 5, 4));
    }

    #[test]
    fn test_loada_one_byte() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LDA, 36, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 0, 0, 0, 5));
    }

    #[test]
    fn test_loada_index_offset() {
        let mut c = Computer::new();
        c.memory[2004] = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.registers.i2 = arch::HalfWord::from_value(4);
        c.run_command(Instruction::new(instructions::OpCode::LDA, 5, 2, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 1, 16, 3, 5, 4));
    }
}
