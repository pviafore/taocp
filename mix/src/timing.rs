use crate::instructions::Instruction;
use crate::instructions::OpCode;

fn get_time_to_run(instruction: Instruction) -> u32{
    match instruction.op_code() {
        OpCode::ADD => 2,
        OpCode::SUB => 2,
        OpCode::LDA => 2,
        OpCode::LDX => 2,
        OpCode::LD1 => 2,
        OpCode::LD2 => 2,
        OpCode::LD3 => 2,
        OpCode::LD4 => 2,
        OpCode::LD5 => 2,
        OpCode::LD6 => 2,
        OpCode::CMPA => 2,
        OpCode::CMPX => 2,
        OpCode::CMP1 => 2,
        OpCode::CMP2 => 2,
        OpCode::CMP3 => 2,
        OpCode::CMP4 => 2,
        OpCode::CMP5 => 2,
        OpCode::CMP6 => 2,
        OpCode::STA => 2,
        OpCode::STX => 2,
        OpCode::ST1 => 2,
        OpCode::ST2 => 2,
        OpCode::ST3 => 2,
        OpCode::ST4 => 2,
        OpCode::ST5 => 2,
        OpCode::ST6 => 2,
        OpCode::STJ => 2,
        OpCode::STZ => 2,
        OpCode::Shift => 2,
        OpCode::HaltNumChar => 10, // halt will actually not take any time at all since we stop
        OpCode::MUL => 10,
        OpCode::DIV => 12,
        OpCode::MOVE => 1 + 2 * instruction.modification() as u32,
        _ => 1
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TimingUnit {
    time_to_run: u32
}

impl TimingUnit {
    pub fn new() -> TimingUnit {
        TimingUnit {
            time_to_run: 0
        }
    }

    pub fn add_time_to_run(&mut self, instruction: Instruction) {
        self.add_raw_time(get_time_to_run(instruction));
    }

    pub fn add_raw_time(&mut self, t: u32){
        self.time_to_run += t;
    }

    pub fn get_time_to_run(self) -> u32{
        self.time_to_run
    }
}
