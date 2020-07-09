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
            instructions::OpCode::LDA => Computer::loada,
            instructions::OpCode::LD1 => Computer::loadi1,
            instructions::OpCode::LD2 => Computer::loadi2,
            instructions::OpCode::LD3 => Computer::loadi3,
            instructions::OpCode::LD4 => Computer::loadi4,
            instructions::OpCode::LD5 => Computer::loadi5,
            instructions::OpCode::LD6 => Computer::loadi6,
            instructions::OpCode::LDX => Computer::loadx,
            instructions::OpCode::LDAN => Computer::loadan,
            instructions::OpCode::LD1N => Computer::loadi1n,
            instructions::OpCode::LD2N => Computer::loadi2n,
            instructions::OpCode::LD3N => Computer::loadi3n,
            instructions::OpCode::LD4N => Computer::loadi4n,
            instructions::OpCode::LD5N => Computer::loadi5n,
            instructions::OpCode::LD6N => Computer::loadi6n,
            instructions::OpCode::LDXN => Computer::loadxn,
            instructions::OpCode::STA => Computer::sta,
            instructions::OpCode::ST1 => Computer::sti1,
            instructions::OpCode::ST2 => Computer::sti2,
            instructions::OpCode::ST3 => Computer::sti3,
            instructions::OpCode::ST4 => Computer::sti4,
            instructions::OpCode::ST5 => Computer::sti5,
            instructions::OpCode::ST6 => Computer::sti6,
            instructions::OpCode::STX => Computer::stx,
            instructions::OpCode::STJ => Computer::stj,
            instructions::OpCode::STZ => Computer::stz,
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
        self.registers.a = self.readmem(instruction)
    }
    
    fn loadx(&mut self, instruction: Instruction) {
        self.registers.x = self.readmem(instruction)
    }
    
    fn loadi1(&mut self, instruction: Instruction) {
        let value = self.readmem(instruction);
        self.registers.i1 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }
    
    fn loadi2(&mut self, instruction: Instruction) {
        let value = self.readmem(instruction);
        self.registers.i2 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }
 
    fn loadi3(&mut self, instruction: Instruction) {
        let value = self.readmem(instruction);
        self.registers.i3 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }

    fn loadi4(&mut self, instruction: Instruction) {
        let value = self.readmem(instruction);
        self.registers.i4 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }

    fn loadi5(&mut self, instruction: Instruction) {
        let value = self.readmem(instruction);
        self.registers.i5 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }

    fn loadi6(&mut self, instruction: Instruction) {
        let value = self.readmem(instruction);
        self.registers.i6 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }

    fn loadan(&mut self, instruction: Instruction) {
       let mut value = self.readmem(instruction);
       value.invert_sign();
       self.registers.a = value;
    }
    
    fn loadxn(&mut self, instruction: Instruction) {
        let mut value = self.readmem(instruction);
        value.invert_sign();
        self.registers.x = value;
    }
    
    fn loadi1n(&mut self, instruction: Instruction) {
        let mut value = self.readmem(instruction);
        value.invert_sign();
        self.registers.i1 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }
    
    fn loadi2n(&mut self, instruction: Instruction) {
        let mut value = self.readmem(instruction);
        value.invert_sign();
        self.registers.i2 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }
 
    fn loadi3n(&mut self, instruction: Instruction) {
        let mut value = self.readmem(instruction);
        value.invert_sign();
        self.registers.i3 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }

    fn loadi4n(&mut self, instruction: Instruction) {
        let mut value = self.readmem(instruction);
        value.invert_sign();
        self.registers.i4 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }

    fn loadi5n(&mut self, instruction: Instruction) {
        let mut value = self.readmem(instruction);
        value.invert_sign();
        self.registers.i5 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }

    fn loadi6n(&mut self, instruction: Instruction) {
        let mut value = self.readmem(instruction);
        value.invert_sign();
        self.registers.i6 = arch::HalfWord::from_values(value.is_positive, value.bytes[3].read(), value.bytes[4].read());
    }

    fn readmem(&self, instruction:Instruction) -> arch::Word {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        if address < 0 || address >= 4000 {
            panic!("Address cannot be negative or out of range")
        }
        let (l,r) = instruction.field_modifier();
        self.memory[address as usize].read_partial_as_word(l,r)
    }

    fn sta(&mut self, instruction: Instruction) {
        self.writemem(instruction, self.registers.a)
    }
    
    fn sti1(&mut self, instruction: Instruction) {
        self.writemem(instruction, arch::Word::from_half_word(self.registers.i1))
    }

    fn sti2(&mut self, instruction: Instruction) {
        self.writemem(instruction, arch::Word::from_half_word(self.registers.i2))
    }

    fn sti3(&mut self, instruction: Instruction) {
        self.writemem(instruction, arch::Word::from_half_word(self.registers.i3))
    }

    fn sti4(&mut self, instruction: Instruction) {
        self.writemem(instruction, arch::Word::from_half_word(self.registers.i4))
    }

    fn sti5(&mut self, instruction: Instruction) {
        self.writemem(instruction, arch::Word::from_half_word(self.registers.i5))
    }

    fn sti6(&mut self, instruction: Instruction) {
        self.writemem(instruction, arch::Word::from_half_word(self.registers.i6))
    }

    fn stx(&mut self, instruction: Instruction) {
        self.writemem(instruction, self.registers.x)
    }
    
    fn stj(&mut self, instruction: Instruction) {
        let mut value = arch::Word::from_half_word(self.registers.j);
        value.is_positive = true;
        self.writemem(instruction, value)
    }
    
    fn stz(&mut self, instruction: Instruction) {
        self.writemem(instruction, arch::Word::new())
    }

    fn writemem(&mut self, instruction: Instruction, value: arch::Word) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        if address < 0 || address >= 4000 {
            panic!("Address cannot be negative or out of range")
        }
       
        let (mut l,r) = instruction.field_modifier();
        if l == 0 {
            self.memory[address as usize].is_positive = value.is_positive;
            l += 1;
        }
        for (index, byte_index) in ((l-1)..r).rev().zip((0..5).rev()) {
            self.memory[address as usize].bytes[index as usize] = value.bytes[byte_index as usize];
        }


    }

    fn get_offset(&self, val: u8) -> i16 {
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
    fn test_loada_field_spec() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LDA, 36, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 0, 0, 0, 5));

        c.memory[2000] = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LDA, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 0, 0, 1, 16, 3));
    }

    #[test]
    fn test_loada_index_offset() {
        let mut c = Computer::new();
        c.memory[2004] = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.registers.i2 = arch::HalfWord::from_value(4);
        c.run_command(Instruction::new(instructions::OpCode::LDA, 5, 2, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 1, 16, 3, 5, 4));
    }
    
    #[test]
    fn test_loadx() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LDX, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.x, arch::Word::from_values(false, 1, 16, 3, 5, 4));
    }

    #[test]
    fn test_loadi1() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD1, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i1, arch::HalfWord::from_values(false, 5, 4));
    }
    
    #[test]
    fn test_loadi2() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 1, 2, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD2, 19, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i2, arch::HalfWord::from_values(true, 1, 2));
    }

    #[test]
    fn test_loadi3() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD3, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i3, arch::HalfWord::from_values(false, 5, 4));
    }
    
    #[test]
    fn test_loadi4() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD4, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i4, arch::HalfWord::from_values(false, 5, 4));
    }
    
    #[test]
    fn test_loadi5() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD5, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i5, arch::HalfWord::from_values(false, 5, 4));
    }
    
    #[test]
    fn test_loadi6() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD6, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i6, arch::HalfWord::from_values(false, 5, 4));
    }

    #[test]
    fn test_loadan() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LDAN, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 1, 16, 3, 5, 4));
    }

    #[test]
    fn test_loadxn() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LDXN, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 1, 16, 3, 5, 4));
    }

    #[test]
    fn test_loadi1n() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD1N, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i1, arch::HalfWord::from_values(true, 5, 4));
    }
    
    #[test]
    fn test_loadi2n() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 1, 2, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD2N, 19, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i2, arch::HalfWord::from_values(false, 1, 2));
    }

    #[test]
    fn test_loadi3n() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD3N, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i3, arch::HalfWord::from_values(true, 5, 4));
    }
    
    #[test]
    fn test_loadi4n() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD4N, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i4, arch::HalfWord::from_values(true, 5, 4));
    }
    
    #[test]
    fn test_loadi5n() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD5N, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i5, arch::HalfWord::from_values(true, 5, 4));
    }
    
    #[test]
    fn test_loadi6n() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::LD6N, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i6, arch::HalfWord::from_values(true, 5, 4));
    }
    
    #[test]
    fn test_sta() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::STA, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(false, 1, 16, 3, 5, 4));
    }

    #[test]
    fn test_sta_with_index_specifier() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.registers.i2 = arch::HalfWord::from_value(4);
        c.run_command(Instruction::new(instructions::OpCode::STA, 5, 2, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2004], arch::Word::from_values(false, 1, 16, 3, 5, 4));
    }
    
    #[test]
    fn test_sta_with_field_specifier() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 16, 3, 5, 4);
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 1);
        c.run_command(Instruction::new(instructions::OpCode::STA, 13, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(false, 1, 16, 3, 5, 4));
        
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 1);
        c.run_command(Instruction::new(instructions::OpCode::STA, 19, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(false, 0, 5, 4, 0, 1));
    }
   
    #[test]
    fn test_sti1() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 1);
        c.registers.i1 = arch::HalfWord::from_values(true, 20, 30);
        c.run_command(Instruction::new(instructions::OpCode::ST1, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(true, 0, 0, 0, 20, 30));
    }

    #[test]
    fn test_sti2() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 1);
        c.registers.i2 = arch::HalfWord::from_values(true, 20, 30);
        c.run_command(Instruction::new(instructions::OpCode::ST2, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(true, 0, 0, 0, 20, 30));
    }

    #[test]
    fn test_sti3() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 1);
        c.registers.i3 = arch::HalfWord::from_values(true, 20, 30);
        c.run_command(Instruction::new(instructions::OpCode::ST3, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(true, 0, 0, 0, 20, 30));
    }

    #[test]
    fn test_sti4() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 1);
        c.registers.i4 = arch::HalfWord::from_values(true, 20, 30);
        c.run_command(Instruction::new(instructions::OpCode::ST4, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(true, 0, 0, 0, 20, 30));
    }

    #[test]
    fn test_sti5() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 1);
        c.registers.i5 = arch::HalfWord::from_values(true, 20, 30);
        c.run_command(Instruction::new(instructions::OpCode::ST5, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(true, 0, 0, 0, 20, 30));
    }

    #[test]
    fn test_sti6() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 1);
        c.registers.i6 = arch::HalfWord::from_values(true, 20, 30);
        c.run_command(Instruction::new(instructions::OpCode::ST6, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(true, 0, 0, 0, 20, 30));
    }

    #[test]
    fn test_stx() {
        let mut c = Computer::new();
        c.registers.x = arch::Word::from_values(false, 1, 16, 3, 5, 4);
        c.run_command(Instruction::new(instructions::OpCode::STX, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(false, 1, 16, 3, 5, 4));
    }
    
    #[test]
    fn test_stj() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 1, 16, 0, 0, 1);
        c.registers.j = arch::HalfWord::from_values(false, 20, 30);
        c.run_command(Instruction::new(instructions::OpCode::STJ, 2, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(true, 20, 30, 0, 0, 1));
    }

    #[test]
    fn test_stz() {
        let mut c = Computer::new();
        c.memory[2000] = arch::Word::from_values(false, 1, 16, 0, 0, 1);
        c.run_command(Instruction::new(instructions::OpCode::STZ, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.memory[2000], arch::Word::from_values(true, 0, 0, 0, 0, 0));
    }
}
