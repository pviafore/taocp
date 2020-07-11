use crate::arch;
use crate::instructions;

use std::cmp;
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
            instructions::OpCode::ADD => Computer::add,
            instructions::OpCode::SUB => Computer::sub,
            instructions::OpCode::MUL => Computer::mul,
            instructions::OpCode::DIV => Computer::div,
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

  

    fn add(&mut self, instruction: Instruction){
        let addend2 = self.readmem(instruction).read();
        self.add_to_register_a(instruction, addend2);
   }
    
    fn sub(&mut self, instruction: Instruction){
        let addend2 = self.readmem(instruction).read();
        self.add_to_register_a(instruction, -1 * addend2);
    }

    fn add_to_register_a(&mut self, instruction: Instruction, addend2: i32){
        let old_is_positive = self.registers.a.is_positive;
        let addend1 = self.registers.a.read();
        let total = addend1 + addend2;
        self.registers.a = arch::Word::from_value(total);
        if (addend1.is_positive() && addend2.is_positive() && self.registers.a.read()  < cmp::max(addend1, addend2)) ||
           (!addend1.is_positive() && !addend2.is_positive() && self.registers.a.read() > cmp::max(addend1, addend2))  {
                self.overflow = true;
        }
        if total == 0 {
            self.registers.a.is_positive = old_is_positive 
        }
    }

    fn mul(&mut self, instruction: Instruction) {
        let multiplicand1 = self.registers.a.read();
        let memory = self.readmem(instruction);
        let multiplicand2 = memory.read();
        let signs_are_same = self.registers.a.is_positive == memory.is_positive;
        let product = multiplicand1.abs() as i64 * multiplicand2.abs() as i64;
        self.registers.a = arch::Word::from_value((product >> 30) as i32);
        self.registers.x = arch::Word::from_value((product % (1 << 30)) as i32);
        self.registers.a.is_positive = signs_are_same;
        self.registers.x.is_positive = signs_are_same;
    }

    fn div(&mut self, instruction:Instruction) {
        let old_register_sign = self.registers.a.is_positive;
        let modifier = if old_register_sign { 1 } else { -1 };
        let dividend = ((self.registers.a.read() as i64) << 30) + modifier * self.registers.x.read().abs() as i64;
        let divisor = self.readmem(instruction).read() as i64;
        if divisor == 0 {
            self.overflow = true;
            return;
        }
        let quotient = dividend / divisor;
        let remainder = dividend % divisor;
        if quotient >= (1i64 << 30){
            self.overflow = true;
        }
        self.registers.a = arch::Word::from_value(quotient as i32); 
        self.registers.x = arch::Word::from_value(remainder as i32); 
        self.registers.x.is_positive = old_register_sign;
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

    #[test]
    fn test_add() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 16, 0, 0, 1);
        c.memory[2000] = arch::Word::from_values(true, 1, 1, 1, 1, 1);
        c.run_command(Instruction::new(instructions::OpCode::ADD, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 2, 17, 1, 1, 2));
    }

    #[test]
    fn test_add_two_negatives() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 2, 16, 3, 4, 1);
        c.memory[2000] = arch::Word::from_values(false, 1, 1, 1, 1, 1);
        c.run_command(Instruction::new(instructions::OpCode::ADD, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 3, 17, 4, 5, 2));
    }

    #[test]
    fn test_add_with_field_specifiers() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 2, 16, 3, 4, 1);
        c.memory[2000] = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::ADD, 19, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 2, 16, 3, 6, 4));
    }
    
    #[test]
    fn test_add_handles_overflow(){
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 63, 63, 63, 63, 63);
        c.memory[2000] = arch::Word::from_values(true, 63, 63, 63, 63, 63);
        c.run_command(Instruction::new(instructions::OpCode::ADD, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 63, 63, 63, 63, 62));
        assert_eq!(c.overflow, true);
    }
    
    #[test]
    fn test_add_handles_negative_overflow(){
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 63, 63, 63, 63, 63);
        c.memory[2000] = arch::Word::from_values(false, 63, 63, 63, 63, 63);
        c.run_command(Instruction::new(instructions::OpCode::ADD, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 63, 63, 63, 63, 62));
        assert_eq!(c.overflow, true);
    }
    
    #[test]
    fn test_add_mixed_signs_cant_overflow(){
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 63, 63, 63, 63, 63);
        c.memory[2000] = arch::Word::from_values(true, 63, 63, 63, 63, 63);
        c.run_command(Instruction::new(instructions::OpCode::ADD, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 0,0,0,0,0));
        assert_eq!(c.overflow, false);
    }
    
    #[test]
    fn test_add_zero_result_preserves_sign(){
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 63, 63, 63, 63, 63);
        c.memory[2000] = arch::Word::from_values(false, 63, 63, 63, 63, 63);
        c.run_command(Instruction::new(instructions::OpCode::ADD, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0,0,0,0,0));
    }
    
    #[test]
    fn test_add_zero_result_preserves_sign_when_addends_are_zero(){
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 0);
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 0);
        c.run_command(Instruction::new(instructions::OpCode::ADD, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0,0,0,0,0));
    }

    #[test]
    fn test_subtract() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 16, 0, 0, 1);
        c.memory[2000] = arch::Word::from_values(true, 1, 1, 1, 1, 1);
        c.run_command(Instruction::new(instructions::OpCode::SUB, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 14, 62, 63, 0));
    }
    
    #[test]
    fn test_subtract_positive_to_negative() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 1, 1, 1, 1);
        c.memory[2000] = arch::Word::from_values(true, 2, 2, 2, 2, 2);
        c.run_command(Instruction::new(instructions::OpCode::SUB, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 1, 1, 1, 1, 1));
    }
    
    #[test]
    fn test_multiply() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 3);
        c.memory[2000] = arch::Word::from_values(true, 2, 2, 2, 2, 2);
        c.run_command(Instruction::new(instructions::OpCode::MUL, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 0, 0, 0, 0));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 6, 6, 6, 6, 6));
        
    }
    
    #[test]
    fn test_multiply_big_num() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 63, 63, 63, 63, 63);
        c.memory[2000] = arch::Word::from_values(true, 63, 63, 63, 63, 63);
        c.run_command(Instruction::new(instructions::OpCode::MUL, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 63, 63, 63, 63, 62));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 0, 0, 0, 0, 1));
        
    }
    
    #[test]
    fn test_multiply_mixed_signs() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 3, 3, 3, 3, 3);
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 6);
        c.run_command(Instruction::new(instructions::OpCode::MUL, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 0, 0, 0, 0, 0));
        assert_eq!(c.registers.x, arch::Word::from_values(false, 18, 18, 18, 18, 18));
    }

    #[test]
    fn test_divide() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 0);
        c.registers.x = arch::Word::from_values(true, 0, 0, 0, 0, 6);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 0, 3);
        c.run_command(Instruction::new(instructions::OpCode::DIV, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 0, 0, 0, 2));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 0, 0, 0, 0, 0));
    }

    #[test]
    fn test_divide_with_remainder() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 0);
        c.registers.x = arch::Word::from_values(true, 0, 0, 0, 0, 7);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 0, 3);
        c.run_command(Instruction::new(instructions::OpCode::DIV, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 0, 0, 0, 2));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 0, 0, 0, 0, 1));
    }
    
    #[test]
    fn test_divide_with_double_negative() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 0, 0, 0, 0, 0);
        c.registers.x = arch::Word::from_values(false, 0, 0, 0, 0, 6);
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 2);
        c.run_command(Instruction::new(instructions::OpCode::DIV, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 0, 0, 0, 3));
        assert_eq!(c.registers.x, arch::Word::from_values(false, 0, 0, 0, 0, 0));
    }
    
    #[test]
    fn test_divide_with_mixed_signs() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 0);
        c.registers.x = arch::Word::from_values(true, 0, 0, 0, 0, 6);
        c.memory[2000] = arch::Word::from_values(false, 0, 0, 0, 0, 2);
        c.run_command(Instruction::new(instructions::OpCode::DIV, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 0, 0, 0, 0, 3));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 0, 0, 0, 0, 0));
    }

    #[test]
    fn test_divide_big_number() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 1);
        c.registers.x = arch::Word::from_values(true, 0, 0, 0, 0, 0);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 1, 0);
        c.run_command(Instruction::new(instructions::OpCode::DIV, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 1, 0, 0, 0, 0));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 0, 0, 0, 0, 0));
    }

    #[test]
    fn test_divide_ignores_rx_sign() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 0);
        c.registers.x = arch::Word::from_values(false, 0, 0, 0, 0, 9);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 0, 3);
        c.run_command(Instruction::new(instructions::OpCode::DIV, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 0, 0, 0, 3));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 0, 0, 0, 0, 0));
    }
    
    #[test]
    fn test_divide_handles_divide_by_zero() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 0);
        c.registers.x = arch::Word::from_values(false, 0, 0, 0, 0, 9);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 0, 0);
        c.run_command(Instruction::new(instructions::OpCode::DIV, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.overflow, true); 
    }
    
    #[test]
    fn test_divide_overflows_during_big_divide() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 0, 0, 0, 0);
        c.registers.x = arch::Word::from_values(false, 0, 0, 0, 0, 9);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 0, 1);
        c.run_command(Instruction::new(instructions::OpCode::DIV, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.overflow, true); 
    }
}
