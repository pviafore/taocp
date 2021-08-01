use crate::arch;
use crate::instructions;
use crate::io;
use crate::timing;

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum DebugCommand {
    NOOP,
    SINGLESTEP,
    BREAK {location: i16},
    CONTINUE,
    SHOWMEM {location: i16},
    BYTES {value: i32},
    HALT
}


#[derive(Debug, PartialEq)]
enum ComparisonIndicator {
    LESS,
    EQUAL,
    GREATER
}

fn compare_half_word(value: arch::HalfWord, rhs: i32, instruction: Instruction) -> ComparisonIndicator {
    compare_word(arch::Word::from_half_word(value), rhs, instruction)
}

fn compare_word(value: arch::Word, rhs: i32, instruction: Instruction) -> ComparisonIndicator {
    let (l,r) = instruction.field_modifier();
    let lhs = value.read_partial_as_word(l, r).read();
    if lhs < rhs {
        ComparisonIndicator::LESS
    }
    else {
        if lhs == rhs { ComparisonIndicator::EQUAL } else { ComparisonIndicator::GREATER }
    }
}

fn print_bytes(value: i32) {
    let word = arch::Word::from_value(value);
    println!("{} {} {} {} {} {}",
             if word.is_positive { "+" } else { "-" },
             word.bytes[0].read(),
             word.bytes[1].read(),
             word.bytes[2].read(),
             word.bytes[3].read(),
             word.bytes[4].read())

}

pub struct Computer {
    registers: Registers,
    trace: bool,
    overflow: bool,
    comparison: ComparisonIndicator,
    memory: Vec<arch::Word>,
    io: io::IO,
    instruction_pointer: arch::HalfWord,
    is_halted: bool,
    timer: timing::TimingUnit,
    last_debug_command: DebugCommand,
    breakpoints: Vec<Box<dyn Fn(&Computer) -> bool>>
}

impl Computer {
    pub fn run_command(&mut self, instruction: Instruction){
        let op = match instruction.op_code() {
            instructions::OpCode::NOP => Computer::no_op,
            instructions::OpCode::ADD => Computer::add,
            instructions::OpCode::SUB => Computer::sub,
            instructions::OpCode::MUL => Computer::mul,
            instructions::OpCode::DIV => Computer::div,
            instructions::OpCode::HaltNumChar => Computer::halt_num_char,
            instructions::OpCode::Shift => Computer::shift,
            instructions::OpCode::MOVE => Computer::mov,
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
            instructions::OpCode::JBUS => Computer::no_op, // we're never busy so no op
            instructions::OpCode::IOC => Computer::ioctl,
            instructions::OpCode::IN => Computer::read,
            instructions::OpCode::OUT => Computer::write,
            instructions::OpCode::JRED => Computer::jred,
            instructions::OpCode::Jump => Computer::jump,
            instructions::OpCode::JumpA => Computer::jumpa,
            instructions::OpCode::JumpI1 => Computer::jumpi1,
            instructions::OpCode::JumpI2 => Computer::jumpi2,
            instructions::OpCode::JumpI3 => Computer::jumpi3,
            instructions::OpCode::JumpI4 => Computer::jumpi4,
            instructions::OpCode::JumpI5 => Computer::jumpi5,
            instructions::OpCode::JumpI6 => Computer::jumpi6,
            instructions::OpCode::JumpX => Computer::jumpx,
            instructions::OpCode::AddressTransferA => Computer::address_transfer_a,
            instructions::OpCode::AddressTransferI1 => Computer::address_transfer_i1,
            instructions::OpCode::AddressTransferI2 => Computer::address_transfer_i2,
            instructions::OpCode::AddressTransferI3 => Computer::address_transfer_i3,
            instructions::OpCode::AddressTransferI4 => Computer::address_transfer_i4,
            instructions::OpCode::AddressTransferI5 => Computer::address_transfer_i5,
            instructions::OpCode::AddressTransferI6 => Computer::address_transfer_i6,
            instructions::OpCode::AddressTransferX => Computer::address_transfer_x,
            instructions::OpCode::CMPA => Computer::cmpa,
            instructions::OpCode::CMP1 => Computer::cmpi1,
            instructions::OpCode::CMP2 => Computer::cmpi2,
            instructions::OpCode::CMP3 => Computer::cmpi3,
            instructions::OpCode::CMP4 => Computer::cmpi4,
            instructions::OpCode::CMP5 => Computer::cmpi5,
            instructions::OpCode::CMP6 => Computer::cmpi6,
            instructions::OpCode::CMPX => Computer::cmpx,
        };
        self.timer.add_time_to_run(instruction);
        op(self, instruction)
    }

    pub fn get_time_to_run(self) -> u32 {
        self.timer.get_time_to_run()
    }

    pub fn load_card_into_memory(&mut self) {
        let first_card = self.io.read(16,0);
        self.write_to_memory(first_card, 0);
    }

    pub fn load_tape(&mut self, tape_contents: Vec<u8>) {
        self.io.load_tape(tape_contents);
    }

    pub fn run(&mut self, use_debugger: bool){
        while !self.is_halted {
            if !use_debugger {
                // keep running blindly without a debugger
                self.run_single_instruction()
            }
            else {
                match self.get_debug_command() {
                    DebugCommand::SINGLESTEP => self.run_single_instruction(),
                    DebugCommand::HALT => self.is_halted = true,
                    DebugCommand::BREAK {location} => self.add_breakpoint(location),
                    DebugCommand::CONTINUE => self.run_single_instruction(),
                    DebugCommand::SHOWMEM {location} => self.print_mem(location),
                    DebugCommand::BYTES {value} => print_bytes(value),
                    DebugCommand::NOOP => ()
                };
            }
        }
    }

    fn print_mem(&self, location: i16) {
        println!("{}", self.memory[location as usize].read());
    }

    fn add_breakpoint(&mut self, location: i16) {
        self.breakpoints.push(Box::new(move |computer| computer.instruction_pointer.read() == location));
        println!("Breakpoint added at location {}", location);
    }

    fn get_debug_command(&mut self) -> DebugCommand {
        if self.last_debug_command == DebugCommand::CONTINUE {
            return if self.breakpoints.iter().any(|breakpoint| breakpoint(self)) {
                self.last_debug_command = DebugCommand::NOOP;
                self.last_debug_command
            }
            else {
                DebugCommand::CONTINUE
            }
        }
        let instruction_pointer = self.instruction_pointer.read() as usize;
        println!();
        println!("Instruction@{:?}: {}", instruction_pointer, Instruction::from_word(self.memory[instruction_pointer]).to_string());
        println!("A: {:?} X {:?} I1: {:?} I2: {:?} I3: {:?} I4: {:?} I5: {:?} I6: {:?}",
                self.registers.a.read(), self.registers.x.read(), self.registers.i1.read(),
                self.registers.i2.read(), self.registers.i3.read(), self.registers.i4.read(),
                self.registers.i5.read(), self.registers.i6.read());
        println!();
        print!(">>> ");
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Needed a string");
        let split: Vec<&str> = input.strip_suffix('\n').unwrap().split(" ").collect();
        self.last_debug_command = match split[0] {
            "q" | "quit" => DebugCommand::HALT,
            "n" | "next" => DebugCommand::SINGLESTEP,
            "b" | "breakpoint" => DebugCommand::BREAK { location: split[1].parse::<i16>().unwrap()},
            "m" | "memory" => DebugCommand::SHOWMEM { location: split[1].parse::<i16>().unwrap()},
            "c" | "continue" => DebugCommand::CONTINUE,
            "B" | "bytes" => DebugCommand::BYTES {value: split[1].parse::<i32>().unwrap()},
            "" => self.last_debug_command,
            _ => DebugCommand::NOOP
        };
        self.last_debug_command

    }


    fn run_single_instruction(&mut self) {
        let instruction = Instruction::from_word(self.memory[self.instruction_pointer.read() as usize]);
        if self.trace {
            println!("{:?}: {:?}", self.instruction_pointer.read(), instruction.to_string());
        }
        self.run_command(instruction);
        if !instruction.is_jump() {
            self.instruction_pointer = arch::HalfWord::from_value(
                self.instruction_pointer.read() + 1
            );
        }
    }


    pub fn write_to_memory(&mut self, data: Vec<arch::Word>, location: usize) {
        for i in 0..(data.len()) {
            self.memory[location+i] = data[i]
        }
    }

    pub fn add_card(&mut self, data: Vec<u8>) {
        self.io.add_card(data)
    }

    pub fn turn_tracing_on(&mut self) {
        self.trace = true;
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
            trace: false,
            overflow: false,
            comparison: ComparisonIndicator::EQUAL,
            memory: Vec::from_iter((0..4000).map(|_| arch::Word::new())),
            io: io::IO::new(),
            instruction_pointer: arch::HalfWord::new(),
            is_halted: false,
            timer: timing::TimingUnit::new(),
            last_debug_command: DebugCommand::NOOP,
            breakpoints: Vec::new()
        }
    }

    fn loada(&mut self, instruction: Instruction) {
        self.registers.a = self.readmem(instruction);
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
            panic!("Address {} cannot be negative or out of range", address)
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
            panic!("Address cannot be negative or out of range: {}", address)
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
        self.add_to_register_a(addend2);
   }

    fn sub(&mut self, instruction: Instruction){

        let addend2 = self.readmem(instruction).read();
        self.add_to_register_a(-1 * addend2);
    }

    fn add_to_register_a(&mut self, addend2: i32){
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

    fn address_transfer_a(&mut self, instruction:Instruction) {
        let value = self.get_address_transfer_value(instruction, self.registers.a.read(), 30);
        self.registers.a = arch::Word::from_value(value);
    }

    fn address_transfer_x(&mut self, instruction:Instruction) {
        let value = self.get_address_transfer_value(instruction, self.registers.x.read(), 30);
        self.registers.x = arch::Word::from_value(value);
    }

    fn address_transfer_i1(&mut self, instruction:Instruction) {
        let value = self.get_address_transfer_value(instruction, self.registers.i1.read() as i32, 12);
        self.registers.i1 = arch::HalfWord::from_value(value as i16);
    }

    fn address_transfer_i2(&mut self, instruction:Instruction) {
        let value = self.get_address_transfer_value(instruction, self.registers.i2.read() as i32, 12);
        self.registers.i2 = arch::HalfWord::from_value(value as i16);
    }

    fn address_transfer_i3(&mut self, instruction:Instruction) {
        let value = self.get_address_transfer_value(instruction, self.registers.i3.read() as i32, 12);
        self.registers.i3 = arch::HalfWord::from_value(value as i16);
    }

    fn address_transfer_i4(&mut self, instruction:Instruction) {
        let value = self.get_address_transfer_value(instruction, self.registers.i4.read() as i32, 12);
        self.registers.i4 = arch::HalfWord::from_value(value as i16);
    }

    fn address_transfer_i5(&mut self, instruction:Instruction) {
        let value = self.get_address_transfer_value(instruction, self.registers.i5.read() as i32, 12);
        self.registers.i5 = arch::HalfWord::from_value(value as i16);
    }

    fn address_transfer_i6(&mut self, instruction:Instruction) {
        let value = self.get_address_transfer_value(instruction, self.registers.i6.read() as i32, 12);
        self.registers.i6 = arch::HalfWord::from_value(value as i16);
    }

    fn get_address_transfer_value(&mut self, instruction: Instruction, base: i32, bits: u8) -> i32 {
        let raw_value = self.get_raw_value(instruction);

        let value = match instruction.modification() {
            0 => base + raw_value as i32,
            1 => base - raw_value as i32,
            2 => raw_value as i32,
            3 => -1 * raw_value as i32,
            _ => panic!("Invalid code for Address Transfer")
        };
        if bits == 30 && (value >= (1 << bits) || value <= -1 * (1 << bits)) {
            self.overflow = true;
        }
        value
    }

    fn get_raw_value(&mut self, instruction:Instruction) -> i16 {
        instruction.address().read() as i16 + self.get_offset(instruction.index_specification()) as i16
    }

    fn cmpa(&mut self, instruction: Instruction) {
        self.comparison = compare_word(self.registers.a, self.readmem(instruction).read(), instruction)
    }

    fn cmpx(&mut self, instruction: Instruction) {
        self.comparison = compare_word(self.registers.x, self.readmem(instruction).read(), instruction)
    }

    fn cmpi1(&mut self, instruction: Instruction) {
        self.comparison = compare_half_word(self.registers.i1, self.readmem(instruction).read(), instruction)
    }
    fn cmpi2(&mut self, instruction: Instruction) {
        self.comparison = compare_half_word(self.registers.i2, self.readmem(instruction).read(), instruction)
    }
    fn cmpi3(&mut self, instruction: Instruction) {
        self.comparison = compare_half_word(self.registers.i3, self.readmem(instruction).read(), instruction)
    }
    fn cmpi4(&mut self, instruction: Instruction) {
        self.comparison = compare_half_word(self.registers.i4, self.readmem(instruction).read(), instruction)
    }
    fn cmpi5(&mut self, instruction: Instruction) {
        self.comparison = compare_half_word(self.registers.i5, self.readmem(instruction).read(), instruction)
    }
    fn cmpi6(&mut self, instruction: Instruction) {
        self.comparison = compare_half_word(self.registers.i6, self.readmem(instruction).read(), instruction)
    }

    fn jred(&mut self, instruction: Instruction) {
        self.jump(Instruction::new(instruction.op_code(), 0, instruction.index_specification(), instruction.address()));
    }
    fn jump(&mut self, instruction: Instruction) {
        let return_address = arch::HalfWord::from_value(
            self.instruction_pointer.read() + 1
        );
        let condition = match instruction.modification() {
            0 => true,
            1 => true,
            2 => self.overflow,
            3 => !self.overflow,
            4 => self.comparison == ComparisonIndicator::LESS,
            5 => self.comparison == ComparisonIndicator::EQUAL,
            6 => self.comparison == ComparisonIndicator::GREATER,
            7 => self.comparison != ComparisonIndicator::LESS,
            8 => self.comparison != ComparisonIndicator::EQUAL,
            9 => self.comparison != ComparisonIndicator::GREATER,
            _ => panic!("Invalid Jump modifier"),
        };
        if instruction.modification() == 2 || instruction.modification() == 3 {
            self.overflow = false;
        }
        if condition {
            let address = instruction.address().read() + self.get_offset(instruction.index_specification());
            self.instruction_pointer = arch::HalfWord::from_value(address);
            if instruction.modification() != 1 {
                self.registers.j = return_address;
            }
        }
        else {
            self.instruction_pointer = return_address;
        }
    }

    fn jumpa(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.jump_value(self.registers.a.read(), instruction.modification(), address)
    }
    fn jumpx(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.jump_value(self.registers.x.read(), instruction.modification(), address)
    }
    fn jumpi1(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.jump_value(self.registers.i1.read() as i32, instruction.modification(), address)
    }
    fn jumpi2(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.jump_value(self.registers.i2.read() as i32, instruction.modification(), address)
    }
    fn jumpi3(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.jump_value(self.registers.i3.read() as i32, instruction.modification(), address)
    }
    fn jumpi4(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.jump_value(self.registers.i4.read() as i32, instruction.modification(), address)
    }
    fn jumpi5(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.jump_value(self.registers.i5.read() as i32, instruction.modification(), address)
    }
    fn jumpi6(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.jump_value(self.registers.i6.read() as i32, instruction.modification(), address)
    }

    fn jump_value(&mut self, value: i32, modifier: u8, address: i16 ) {
        let condition = match modifier {
            0 => value < 0,
            1 => value == 0,
            2 => value > 0,
            3 => value >= 0,
            4 => value != 0,
            5 => value <= 0,
            _ => panic!("Invalid Register Jump Condition")
        };
        let return_address = arch::HalfWord::from_value(
            self.instruction_pointer.read() + 1
        );
        if condition {
            self.instruction_pointer = arch::HalfWord::from_value(address);
            self.registers.j = return_address;
        }
        else {
            // need to mess with instruction_pointer because we may not jump
            self.instruction_pointer = return_address;
        }
    }

    fn shift(&mut self, instruction: Instruction) {
        let mut bytes_to_shift = instruction.address().read() + self.get_offset(instruction.index_specification());
        if instruction.modification() == 0 {
            for index in (bytes_to_shift..5).rev() {
                self.registers.a.bytes[4 - index as usize] = self.registers.a.bytes[4 - (index - bytes_to_shift) as usize];
            }
            for index in 0..bytes_to_shift {
                self.registers.a.bytes[4 - index as usize] = arch::Byte::new(0);
            }

        }
        else if instruction.modification() == 1 {
            for index in bytes_to_shift..5 {
                self.registers.a.bytes[index as usize] = self.registers.a.bytes[(index - bytes_to_shift) as usize];
            }
            for index in 0..bytes_to_shift {
                self.registers.a.bytes[index as usize] = arch::Byte::new(0);
            }
        }
        else if instruction.modification() == 2 || instruction.modification() == 4 {
            if instruction.modification() == 4 {
                bytes_to_shift = bytes_to_shift % 10;
            }
            let dword = ((self.registers.a.read().abs() as u64) << 30) + self.registers.x.read().abs() as u64;
            for index in (bytes_to_shift..10).rev() {
                let amount_to_shift = 6 * (index - bytes_to_shift);
                let target_value = arch::Byte::new((dword >> amount_to_shift) as u8);
                let reg: &mut arch::Word = if 9 - index < 5 { &mut self.registers.a } else { &mut self.registers.x };
                reg.bytes[((9 - index) % 5) as usize] = target_value;
            }
            for index in (10 - bytes_to_shift)..10 {
                let reg: &mut arch::Word = if index < 5 { &mut self.registers.a } else { &mut self.registers.x };
                let target_value = if instruction.modification() == 2 { 0 } else { (dword >> 6 * (9 - (index - (10 - bytes_to_shift)))) as u8 };
                reg.bytes[(index % 5) as usize] = arch::Byte::new(target_value);
            }
        }
        else if instruction.modification() == 3 || instruction.modification() == 5 {
            if instruction.modification() == 5 {
                bytes_to_shift = bytes_to_shift % 10;
            }
            let dword = ((self.registers.a.read().abs() as u64) << 30) + self.registers.x.read().abs() as u64;
            for index in (bytes_to_shift..10).rev() {
                let amount_to_shift = 6 * index;
                let target_value = arch::Byte::new((dword >> amount_to_shift) as u8);
                let reg: &mut arch::Word = if index < 5 { &mut self.registers.a } else { &mut self.registers.x };
                reg.bytes[((9 - (index - bytes_to_shift))  % 5) as usize] = target_value;
            }
            for index in 0..bytes_to_shift {
                let reg: &mut arch::Word = if index < 5 { &mut self.registers.a } else { &mut self.registers.x };
                let target_value = if instruction.modification() == 3 { 0 } else { (dword >> 6 * (bytes_to_shift - index - 1))  as u8 };
                reg.bytes[(index % 5) as usize] = arch::Byte::new(target_value);
            }
        }
        else {
            panic!("Invalid shift modification");
        }
    }

    fn mov(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        for index in 0..(instruction.modification() as i16) {
            self.memory[(index + self.registers.i1.read()) as usize] = self.memory[(address + index) as usize];
        }
        self.registers.i1 = arch::HalfWord::from_value(self.registers.i1.read() + instruction.modification() as i16);
    }

    fn no_op(&mut self, _instruction: Instruction) {

    }

    fn halt_num_char(&mut self, instruction: Instruction) {
        if instruction.modification() == 2 {
            self.is_halted = true;
        }
        else if instruction.modification() == 0 {
            let val1 = self.registers.a.bytes;
            let val2 = self.registers.x.bytes;
            let num =
                (val1[0].read() % 10) as i32 * 1000000000 +
                (val1[1].read() % 10) as i32 * 100000000 +
                (val1[2].read() % 10) as i32 * 10000000 +
                (val1[3].read() % 10) as i32 * 1000000 +
                (val1[4].read() % 10) as i32 * 100000 +
                (val2[0].read() % 10) as i32 * 10000 +
                (val2[1].read() % 10) as i32 * 1000 +
                (val2[2].read() % 10) as i32 * 100 +
                (val2[3].read() % 10) as i32 * 10 +
                (val2[4].read() % 10) as i32;
            let modifier = if self.registers.a.is_positive { 1 } else { -1 };
            self.registers.a = arch::Word::from_value(modifier * num);
        }

        else if instruction.modification() == 1 {
            let value = self.registers.a.read().abs();
            self.registers.a.bytes[0] = arch::Byte::new(30 + ((value / 1000000000) % 10) as u8);
            self.registers.a.bytes[1] = arch::Byte::new(30 + ((value / 100000000) % 10) as u8);
            self.registers.a.bytes[2] = arch::Byte::new(30 + ((value / 10000000) % 10) as u8);
            self.registers.a.bytes[3] = arch::Byte::new(30 + ((value / 1000000) % 10) as u8);
            self.registers.a.bytes[4] = arch::Byte::new(30 + ((value / 100000) % 10) as u8);
            self.registers.x.bytes[0] = arch::Byte::new(30 + ((value / 10000) % 10) as u8);
            self.registers.x.bytes[1] = arch::Byte::new(30 + ((value / 1000) % 10) as u8);
            self.registers.x.bytes[2] = arch::Byte::new(30 + ((value / 100) % 10) as u8);
            self.registers.x.bytes[3] = arch::Byte::new(30 + ((value / 10) % 10) as u8);
            self.registers.x.bytes[4] = arch::Byte::new(30 + (value  % 10) as u8);
        }
        else {
            panic!("Unknown halt/num/char")
        }
    }

    fn read(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        let values = self.io.read(instruction.modification(), self.registers.x.read().abs() as u32);
        self.memory.splice(address as usize..(address as usize + values.len()), values.iter().cloned());
    }

    fn write(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.io.write(instruction.modification(), self.registers.x.read().abs() as u32, &self.memory[address as usize..])
    }

    fn ioctl(&mut self, instruction: Instruction) {
        let address = instruction.address().read() + self.get_offset(instruction.index_specification());
        self.io.ioctl(instruction.modification(), address, self.registers.x.read().abs() as u32);

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

    #[test]
    fn test_enta(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferA, 2, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a.read(), 2000);
    }

    #[test]
    fn test_enta_indexed(){
        let mut c = Computer::new();
        c.registers.i1 = arch::HalfWord::from_value(300);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferA, 2, 1, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a.read(), 2300);
    }

    #[test]
    fn test_entx(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferX, 2, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.x.read(), 2000);
    }

    #[test]
    fn test_enti1(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI1, 2, 1, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i1.read(), 2000);
    }

    #[test]
    fn test_enti2(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI2, 2, 1, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i2.read(), 2000);
    }

    #[test]
    fn test_enti3(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI3, 2, 1, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i3.read(), 2000);
    }

    #[test]
    fn test_enti4(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI4, 2, 1, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i4.read(), 2000);
    }

    #[test]
    fn test_enti5(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI5, 2, 1, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i5.read(), 2000);
    }

    #[test]
    fn test_enti6(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI6, 2, 1, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i6.read(), 2000);
    }

    #[test]
    fn test_enna(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferA, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a.read(), -2000);
    }

    #[test]
    fn test_ennx(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferX, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.x.read(), -2000);
    }

    #[test]
    fn test_enni1(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI1, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i1.read(), -2000);
    }

    #[test]
    fn test_enni2(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI2, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i2.read(), -2000);
    }

    #[test]
    fn test_enni3(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI3, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i3.read(), -2000);
    }

    #[test]
    fn test_enni4(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI4, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i4.read(), -2000);
    }

    #[test]
    fn test_enni5(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI5, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i5.read(), -2000);
    }

    #[test]
    fn test_enni6(){
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI6, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i6.read(), -2000);
    }

    #[test]
    fn test_inca(){
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferA, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a.read(), 2005);
    }

    #[test]
    fn test_incx(){
        let mut c = Computer::new();
        c.registers.x = arch::Word::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferX, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.x.read(), 2005);
    }

    #[test]
    fn test_inci1(){
        let mut c = Computer::new();
        c.registers.i1 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI1, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i1.read(), 2005);
    }

    #[test]
    fn test_inci2(){
        let mut c = Computer::new();
        c.registers.i2 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI2, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i2.read(), 2005);
    }

    #[test]
    fn test_inci3(){
        let mut c = Computer::new();
        c.registers.i3 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI3, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i3.read(), 2005);
    }

    #[test]
    fn test_inci4(){
        let mut c = Computer::new();
        c.registers.i4 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI4, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i4.read(), 2005);
    }

    #[test]
    fn test_inci5(){
        let mut c = Computer::new();
        c.registers.i5 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI5, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i5.read(), 2005);
    }

    #[test]
    fn test_inci6(){
        let mut c = Computer::new();
        c.registers.i6 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI6, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i6.read(), 2005);
    }

    #[test]
    fn test_deca(){
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferA, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.a.read(), -1995);
    }

    #[test]
    fn test_decx(){
        let mut c = Computer::new();
        c.registers.x = arch::Word::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferX, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.x.read(), -1995);
    }

    #[test]
    fn test_deci1(){
        let mut c = Computer::new();
        c.registers.i1 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI1, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i1.read(), -1995);
    }

    #[test]
    fn test_deci2(){
        let mut c = Computer::new();
        c.registers.i2 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI2, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i2.read(), -1995);
    }

    #[test]
    fn test_deci3(){
        let mut c = Computer::new();
        c.registers.i3 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI3, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i3.read(), -1995);
    }

    #[test]
    fn test_deci4(){
        let mut c = Computer::new();
        c.registers.i4 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI4, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i4.read(), -1995);
    }

    #[test]
    fn test_deci5(){
        let mut c = Computer::new();
        c.registers.i5 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI5, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i5.read(), -1995);
    }

    #[test]
    fn test_deci6(){
        let mut c = Computer::new();
        c.registers.i6 = arch::HalfWord::from_value(5);
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferI6, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.registers.i6.read(), -1995);
    }

    #[test]
    fn test_inca_overflows_negative(){
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_value(-64 * 64 * 64 * 64 * 64 + 1 );
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferA, 0, 0, arch::HalfWord::from_value(-1)));
        assert_eq!(c.overflow, true);
    }

    #[test]
    fn test_cmpa(){
        let mut c = Computer::new();
        c.comparison = ComparisonIndicator::LESS;
        c.registers.a = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.memory[2000] = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::CMPA, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.comparison, ComparisonIndicator::EQUAL);
    }

    #[test]
    fn test_cmpx(){
        let mut c = Computer::new();
        c.registers.x = arch::Word::from_values(true, 1, 2, 3, 4, 4);
        c.memory[2000] = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::CMPA, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.comparison, ComparisonIndicator::LESS);
    }

    #[test]
    fn test_cmpi1 (){
        let mut c = Computer::new();
        c.registers.i1 = arch::HalfWord::from_values(true, 4, 6);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::CMP1, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.comparison, ComparisonIndicator::GREATER);
    }

    #[test]
    fn test_cmpi2 (){
        let mut c = Computer::new();
        c.registers.i2 = arch::HalfWord::from_values(true, 4, 4);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::CMP2, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.comparison, ComparisonIndicator::LESS);
    }

    #[test]
    fn test_cmpi3 (){
        let mut c = Computer::new();
        c.registers.i3 = arch::HalfWord::from_values(true, 4, 4);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::CMP3, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.comparison, ComparisonIndicator::EQUAL);
    }

    #[test]
    fn test_cmpi4 (){
        let mut c = Computer::new();
        c.registers.i4 = arch::HalfWord::from_values(true, 4, 4);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 0, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::CMP4, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.comparison, ComparisonIndicator::EQUAL);
    }

    #[test]
    fn test_cmpi5 (){
        let mut c = Computer::new();
        c.registers.i5 = arch::HalfWord::from_values(true, 5, 4);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 1, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::CMP5, 36, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.comparison, ComparisonIndicator::GREATER);
    }

    #[test]
    fn test_cmpi6 (){
        let mut c = Computer::new();
        c.registers.i6 = arch::HalfWord::from_values(true, 5, 4);
        c.memory[2000] = arch::Word::from_values(true, 0, 0, 1, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::CMP6, 36, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.comparison, ComparisonIndicator::GREATER);
    }

    #[test]
    fn test_jmp() {
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::Jump, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 1);
    }

    #[test]
    fn test_jsj() {
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::Jump, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 0);
    }

    #[test]
    fn test_jov() {
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::Jump, 2, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.overflow = true;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 2, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
        assert_eq!(c.overflow, false);
    }

    #[test]
    fn test_jnov() {
        let mut c = Computer::new();
        c.overflow = true;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);
        assert_eq!(c.overflow, false);

        c.run_command(Instruction::new(instructions::OpCode::Jump, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_jl() {
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::Jump, 4, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.comparison = ComparisonIndicator::LESS;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 4, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_jg() {
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::Jump, 6, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.comparison = ComparisonIndicator::GREATER;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 6, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_je() {
        let mut c = Computer::new();
        c.comparison = ComparisonIndicator::LESS;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.comparison = ComparisonIndicator::EQUAL;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_jge() {
        let mut c = Computer::new();
        c.comparison = ComparisonIndicator::LESS;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 7, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.comparison = ComparisonIndicator::EQUAL;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 7, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);

        c.comparison = ComparisonIndicator::GREATER;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 7, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2001);
    }

    #[test]
    fn test_jle() {
        let mut c = Computer::new();
        c.comparison = ComparisonIndicator::GREATER;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 9, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.comparison = ComparisonIndicator::EQUAL;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 9, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);

        c.comparison = ComparisonIndicator::LESS;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 9, 0, arch::HalfWord::from_value(3000)));
        assert_eq!(c.instruction_pointer.read(), 3000);
        assert_eq!(c.registers.j.read(), 2001);
    }

    #[test]
    fn test_jne() {
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::Jump, 8, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.comparison = ComparisonIndicator::LESS;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 8, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);

        c.comparison = ComparisonIndicator::GREATER;
        c.run_command(Instruction::new(instructions::OpCode::Jump, 8, 0, arch::HalfWord::from_value(3000)));
        assert_eq!(c.instruction_pointer.read(), 3000);
        assert_eq!(c.registers.j.read(), 2001);
    }

    #[test]
    fn test_jan() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.a = arch::Word::from_values(false, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_jaz() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.a = arch::Word::from_values(false, 0, 0, 0, 0, 0);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 1, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_jap() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 2, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 1);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 2, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_jann() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 0);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 3, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_janz() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 0, 0, 0, 0, 0);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 4, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 1);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 4, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_janp() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 0, 0, 1);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.a = arch::Word::from_values(false, 0, 0, 0, 0, 0);
        c.run_command(Instruction::new(instructions::OpCode::JumpA, 5, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_jxn() {
        let mut c = Computer::new();
        c.registers.x = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpX, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.x = arch::Word::from_values(false, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpX, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_ji1n() {
        let mut c = Computer::new();
        c.registers.i1 = arch::HalfWord::from_values(true, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI1, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.i1 = arch::HalfWord::from_values(false, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI1, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_ji2n() {
        let mut c = Computer::new();
        c.registers.i2 = arch::HalfWord::from_values(true, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI2, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.i2 = arch::HalfWord::from_values(false, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI2, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_ji3n() {
        let mut c = Computer::new();
        c.registers.i3 = arch::HalfWord::from_values(true, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI3, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.i3 = arch::HalfWord::from_values(false, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI3, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }
    #[test]
    fn test_ji4n() {
        let mut c = Computer::new();
        c.registers.i4 = arch::HalfWord::from_values(true, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI4, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.i4 = arch::HalfWord::from_values(false, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI4, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_ji5n() {
        let mut c = Computer::new();
        c.registers.i5 = arch::HalfWord::from_values(true, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI5, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.i5 = arch::HalfWord::from_values(false, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI5, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_ji6n() {
        let mut c = Computer::new();
        c.registers.i6 = arch::HalfWord::from_values(true, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI6, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 1);
        assert_eq!(c.registers.j.read(), 0);

        c.registers.i6 = arch::HalfWord::from_values(false, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::JumpI6, 0, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.instruction_pointer.read(), 2000);
        assert_eq!(c.registers.j.read(), 2);
    }

    #[test]
    fn test_sla() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::Shift, 0, 0, arch::HalfWord::from_value(2)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 3, 4, 5, 0, 0));
    }

    #[test]
    fn test_sra() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.run_command(Instruction::new(instructions::OpCode::Shift, 1, 0, arch::HalfWord::from_value(3)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 0, 0, 1, 2));
    }

    #[test]
    fn test_slax() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.registers.x = arch::Word::from_values(true, 6, 7, 8, 9, 10);
        c.run_command(Instruction::new(instructions::OpCode::Shift, 2, 0, arch::HalfWord::from_value(6)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 7, 8, 9, 10, 0));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 0, 0, 0, 0, 0));
    }

    #[test]
    fn test_srax() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.registers.x = arch::Word::from_values(true, 6, 7, 8, 9, 10);
        c.run_command(Instruction::new(instructions::OpCode::Shift, 3, 0, arch::HalfWord::from_value(6)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 0, 0, 0, 0, 0));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 0, 1, 2, 3, 4));
    }

    #[test]
    fn test_slc() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.registers.x = arch::Word::from_values(true, 6, 7, 8, 9, 10);
        c.run_command(Instruction::new(instructions::OpCode::Shift, 4, 0, arch::HalfWord::from_value(16)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 7, 8, 9, 10, 1));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 2, 3, 4, 5, 6));
    }

    #[test]
    fn test_src() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.registers.x = arch::Word::from_values(true, 6, 7, 8, 9, 10);
        c.run_command(Instruction::new(instructions::OpCode::Shift, 5, 0, arch::HalfWord::from_value(16)));
        assert_eq!(c.registers.a, arch::Word::from_values(true, 5, 6, 7, 8, 9));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 10, 1, 2, 3, 4));
    }

    #[test]
    fn test_move() {
        let mut c = Computer::new();
        c.memory[999] = arch::Word::from_values(true, 1, 2, 3, 4, 5);
        c.memory[1000] = arch::Word::from_values(false, 11, 12, 13, 14, 15);
        c.memory[1001] = arch::Word::from_values(true, 21, 22, 23, 24, 25);
        c.memory[1002] = arch::Word::from_values(false, 31, 32, 33, 34, 35);
        c.registers.i1 = arch::HalfWord::from_value(999);
        c.run_command(Instruction::new(instructions::OpCode::MOVE, 3, 0, arch::HalfWord::from_value(1000)));
        assert_eq!(c.memory[999], arch::Word::from_values(false, 11, 12, 13, 14, 15));
        assert_eq!(c.memory[1000], arch::Word::from_values(true, 21, 22, 23, 24, 25));
        assert_eq!(c.memory[1001], arch::Word::from_values(false, 31, 32, 33, 34, 35));
        assert_eq!(c.memory[1002], arch::Word::from_values(false, 31, 32, 33, 34, 35));
        assert_eq!(c.registers.i1, arch::HalfWord::from_value(1002));
    }

    #[test]
    fn test_hlt() {
        let mut c = Computer::new();
        assert_eq!(c.is_halted, false);
        c.run_command(Instruction::new(instructions::OpCode::HaltNumChar, 2, 0, arch::HalfWord::from_value(1000)));
        assert_eq!(c.is_halted, true);
    }

    #[test]
    fn test_in() {
        let mut c = Computer::new();
        use std::iter::FromIterator;
        let values = Vec::from_iter((0..100).map(|i| arch::Word::from_value(i)));
        c.io.write(2, 0, &values[..]);
        c.io.ioctl(2, 0, 0);

        c.run_command(Instruction::new(instructions::OpCode::IN, 2, 0, arch::HalfWord::from_value(1000)));

        for index in 0..100{
            assert_eq!(c.memory[1000+index], arch::Word::from_value(index as i32));
        }
    }

    #[test]
    fn test_out() {
        let mut c = Computer::new();
        for index in 0..100{
            c.memory[1000+index] = arch::Word::from_value(index as i32);
        }

        c.run_command(Instruction::new(instructions::OpCode::OUT, 2, 0, arch::HalfWord::from_value(1000)));
        c.io.ioctl(2, 0, 0);

        let values = c.io.read(2, 0);
        for index in 0..100{
            assert_eq!(values[index], arch::Word::from_value(index as i32));
        }
    }

    #[test]
    fn test_ioctl() {
        let mut c = Computer::new();
        for index in 0..200{
            c.memory[1000+index] = arch::Word::from_value(index as i32);
        }

        c.run_command(Instruction::new(instructions::OpCode::OUT, 2, 0, arch::HalfWord::from_value(1000)));
        c.run_command(Instruction::new(instructions::OpCode::OUT, 2, 0, arch::HalfWord::from_value(1100)));
        c.run_command(Instruction::new(instructions::OpCode::IOC, 2, 0, arch::HalfWord::from_value(-150)));

        let values = c.io.read(2, 0);
        for index in 0..100{
            assert_eq!(values[index], arch::Word::from_value(50 + index as i32));
        }
    }

    #[test]
    fn test_num() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(true, 0, 0, 31, 32, 39);
        c.registers.x = arch::Word::from_values(true, 37, 57, 47, 30, 30);
        c.run_command(Instruction::new(instructions::OpCode::HaltNumChar, 0, 0, arch::HalfWord::from_value(0)));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 37, 57, 47, 30, 30));
        assert_eq!(c.registers.a, arch::Word::from_value(12977700));
    }

    #[test]
    fn test_num_overflow() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_values(false, 11, 0, 27, 63, 57);
        c.registers.x = arch::Word::from_values(true, 4, 12, 41, 22, 34);
        c.run_command(Instruction::new(instructions::OpCode::HaltNumChar, 0, 0, arch::HalfWord::from_value(0)));
        assert_eq!(c.registers.x, arch::Word::from_values(true, 4, 12, 41, 22, 34));
        assert_eq!(c.registers.a, arch::Word::from_value(-300));
    }

    #[test]
    fn test_char() {
        let mut c = Computer::new();
        c.registers.a = arch::Word::from_value(-12977700);
        c.registers.x = arch::Word::from_values(false, 4, 12, 41, 22, 34);
        c.run_command(Instruction::new(instructions::OpCode::HaltNumChar, 1, 0, arch::HalfWord::from_value(0)));
        assert_eq!(c.registers.a, arch::Word::from_values(false, 30, 30, 31, 32, 39));
        assert_eq!(c.registers.x, arch::Word::from_values(false, 37, 37, 37, 30, 30));
    }

    #[test]
    fn time_computer() {
        let mut c = Computer::new();
        c.run_command(Instruction::new(instructions::OpCode::AddressTransferA, 1, 0, arch::HalfWord::from_value(1000)));
        c.run_command(Instruction::new(instructions::OpCode::STA, 5, 0, arch::HalfWord::from_value(2000)));
        c.run_command(Instruction::new(instructions::OpCode::MUL, 5, 0, arch::HalfWord::from_value(2000)));
        c.run_command(Instruction::new(instructions::OpCode::MOVE, 60, 0, arch::HalfWord::from_value(2000)));
        assert_eq!(c.timer.get_time_to_run(), 134);
    }

}
