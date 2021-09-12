use crate::arch;
use crate::chartable;
use crate::instructions;

use std::collections::HashMap;

struct ProgramData {
    start_location: Option<usize>,
    symbol_table: HashMap<String, i16>,
    label_table: HashMap<String, usize>,
    instruction_lines: Vec<String>,
    symbolic_constants: Vec<String>,
    local_symbols: HashMap<String, Vec<usize>>,
    location_lookup: HashMap<usize, usize>
}

impl ProgramData {
    pub fn new() -> ProgramData {
        ProgramData {
            start_location: None,
            symbol_table: HashMap::new(),
            label_table: HashMap::new(),
            instruction_lines: Vec::new(),
            symbolic_constants: Vec::new(),
            local_symbols: HashMap::new(),
            location_lookup: HashMap::new()
        }
    }

    pub fn get_debug_data(&self) -> String {
        format!("Start Location: {:?}\n\nSymbol Table: {:?}\n\nLabel Table: {:?}\n\n Local Symbol Table {:?}",
                 self.start_location.unwrap(), self.symbol_table, self.label_table, self.local_symbols)
    }

    pub fn add_symbolic_constant(&mut self, text: &str) -> String {
        let index = self.symbolic_constants.iter().position(|x| *x == text.to_string());
        match index {
            Some(idx) => format!("con{}", idx),
            None => {
                let name = format!("con{}", self.symbolic_constants.len());
                self.symbolic_constants.push(text.to_string());
                name
            }
        }
    }

    pub fn get_instructions(&self) -> Vec<String> {
        let mut vec: Vec<String> = Vec::new();
        for index in *self.location_lookup.keys().min().unwrap() as u32..=*self.location_lookup.keys().max().unwrap() as u32 {
            if self.location_lookup.contains_key(&(index as usize)) {
                let line_index: usize = *self.location_lookup.get(&(index as usize)).unwrap();
                vec.push(self.instruction_lines[line_index].clone())
            }
            else {
                vec.push(" NOP".to_string());
            }
        }
        vec
    }

    pub fn get_min_address(&self) -> usize {
        return *self.location_lookup.keys().min().unwrap_or(&100)
    }
}

fn _is_local_symbol(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    chars.len() == 2 && chars[0].is_digit(10) && (chars[1] == 'H' || chars[1] == 'F' || chars[1] == 'B')
}

pub fn assemble(lines: Vec<String>, verbose: bool) -> (Vec<u8>, usize, Option<usize>) {
    let program_data = get_program_data(lines);
    if verbose {
        println!("{}", program_data.get_debug_data());
    }
    let words: Vec<arch::Word> = program_data.get_instructions().iter()
                                             .filter(|x| !x.is_empty())
                                             .enumerate()
                                             .map(|(i, x)| to_instruction(&x, i+program_data.get_min_address(), &program_data))
                                             .collect();
    let word_bytes: Vec<Vec<u8>> = words.iter().map(_make_bytes).collect();
    let bytes: Vec<u8> = word_bytes.into_iter().flatten().collect();
    (bytes, program_data.get_min_address(), program_data.start_location)
}

fn get_program_data(lines: Vec<String>) -> ProgramData {
    let mut program_data = ProgramData::new();
    let mut index = 0;
    let mut relative_index = 0;
    let mut last_location: Option<usize> = None;
    for line in lines {
        if line.is_empty() || line.starts_with('*'){
            continue
        }
        let (label, op, address) = tokenize(&line);
        if op == "EQU" {
            let val = _evaluate(address, last_location.unwrap_or(100)+relative_index, &program_data);
            program_data.symbol_table.insert(String::from(label), val.parse::<i16>().unwrap());
        }
        else if op == "ORIG" {
            let location = _evaluate(address, last_location.unwrap_or(100)+relative_index, &program_data).parse::<usize>().ok();
            program_data.label_table.insert(String::from(label), last_location.unwrap_or(100)+relative_index);

            match program_data.start_location {
                None => {
                    program_data.start_location = location
                }
                _ => {}
            }
            relative_index = 0;
            last_location = location;
        }
        else if op == "END" {
            program_data.start_location = Some(*program_data.label_table.get(address).unwrap());
            for (idx, symbolic_constant) in program_data.symbolic_constants.iter().enumerate() {
                program_data.instruction_lines.push(format!("con{} CON {}", idx, symbolic_constant));
                program_data.location_lookup.insert(last_location.unwrap()+relative_index, index);
                program_data.label_table.insert(format!("con{}", idx), last_location.unwrap() + relative_index);
                relative_index += 1;
                index += 1;
            }
            relative_index += 1;
            program_data.instruction_lines.push(" HLT".to_string());
            program_data.location_lookup.insert(last_location.unwrap()+relative_index, index);
            index += 1;
        }
        else {
            if label != "" {
                if _is_local_symbol(label) {
                    let first_digit = label.chars().next().unwrap().to_string();
                    if !program_data.local_symbols.contains_key(&first_digit) {
                        program_data.local_symbols.insert(first_digit.clone(), Vec::new());
                    }
                    program_data.local_symbols.get_mut(&first_digit).unwrap().push(last_location.unwrap() + relative_index);
                }
                else {
                    program_data.label_table.insert(label.to_string(), last_location.unwrap() + relative_index);
                }
            }
            if address.starts_with('=') && address.ends_with('=') && address.len() > 1 {
                let spl: Vec<&str> = address.split('=').collect();
                let symbolic_constant = program_data.add_symbolic_constant(spl[1]);
                program_data.instruction_lines.push(format!("{} {} {}", label, op, symbolic_constant))
            }
            else {
                program_data.instruction_lines.push(line);
            }
            program_data.location_lookup.insert(last_location.unwrap() + relative_index, index);
            index += 1;
            relative_index += 1;
        }
    }
    program_data
}

fn _make_bytes(word: &arch::Word) -> Vec<u8> {
    let mut u8s = word.as_u8s();
    let sign = if word.is_positive { '+' } else { '-' };
    u8s.insert(0, sign as u8);
    u8s

}

fn to_instruction(line: &str, line_index: usize, program_data: &ProgramData) -> arch::Word {
    // tokenize into words (with empty spaces)
    // strip the 1..n  empty spaces
    // 1st word is ignored (for now)
    // 2nd word is op code
    // 3rd word is address,I(F)
    let (_label, op, address_string) = tokenize(line);
    match op {
        "CON" => {
            arch::Word::from_value(_evaluate(address_string, line_index, program_data).parse::<i32>().unwrap())
        },
        "ALF" => {
            _parse_alphabetic(line)
        },
        _ => {
            let (address, index, modifier) = parse_address_string(op, address_string, line_index, program_data );
            let opcode = instructions::str_to_opcode(op);
            arch::Word::from_values(address.is_positive, address.bytes[0].read(), address.bytes[1].read(),
                                    index, modifier, opcode as u8)
        }
    }
}

fn _parse_alphabetic(line: &str) -> arch::Word{
    let text: Vec<&str> = line.split("ALF ").collect();
    let letters: Vec<char> = text[1].chars().collect();
    if letters[0] == ' ' {
        arch::Word::from_values(true, chartable::to_u8(*letters.get(1).unwrap_or(&' ')),
                                      chartable::to_u8(*letters.get(2).unwrap_or(&' ')),
                                      chartable::to_u8(*letters.get(3).unwrap_or(&' ')),
                                      chartable::to_u8(*letters.get(4).unwrap_or(&' ')),
                                      chartable::to_u8(*letters.get(5).unwrap_or(&' ')))
    }
    else {
        arch::Word::from_values(true, chartable::to_u8(*letters.get(0).unwrap_or(&' ')),
                                      chartable::to_u8(*letters.get(1).unwrap_or(&' ')),
                                      chartable::to_u8(*letters.get(2).unwrap_or(&' ')),
                                      chartable::to_u8(*letters.get(3).unwrap_or(&' ')),
                                      chartable::to_u8(*letters.get(4).unwrap_or(&' ')))
    }
}

fn tokenize(line: &str) -> (&str, &str, &str) {
    // this routine does not handle extra remarks
    let spl: Vec<&str> = line.split(" ").collect();
    let label = spl[0];
    let filtered_split: Vec<&str> = spl[1..].iter().cloned().filter(|x| *x != "").collect();
    let address = if filtered_split.len() > 1 { filtered_split[1] } else { "" };
    (label, filtered_split[0], address)
}

fn parse_address_string(op: &str, address_string: &str, line_index: usize, program_data: &ProgramData) -> (arch::HalfWord, u8, u8) {
    let spl: Vec<&str> = address_string.split(",").collect();
    if spl.len() == 0 || spl[0] == "" {
        match op {
            "HLT" => (arch::HalfWord::new(), 0, 2),
            "NOP" => (arch::HalfWord::new(), 0, 0),
            "NUM" => (arch::HalfWord::new(), 0, 0),
            "CHAR" => (arch::HalfWord::new(), 0, 1),
            _ => (arch::HalfWord::new(), 0, 5)
        }
    }
    else {
        let address = _get_address(&spl, line_index, program_data);
        let index = _get_index(&spl);
        let modifier = _get_modifier(op, spl, program_data);
        (address, index, modifier)
    }
}

fn _get_address(spl: &Vec<&str>, index: usize, program_data: &ProgramData) -> arch::HalfWord {
    let text = if spl[0].contains('(') {
        let address_split: Vec<&str> = spl[0].split('(').collect();
        address_split[0]
    }
    else {
        spl[0]
    };
    let evaluated = if text.chars().any(|c| c == '+' || c == '*' || c == '-' || c == '/') {
        _evaluate(text, index, program_data)
    }
    else {
        String::from(text)
    };
    let val = if program_data.symbol_table.contains_key(&evaluated) {
        program_data.symbol_table.get(&evaluated).unwrap().clone()
    }
    else {
        if _is_local_symbol(&evaluated) {
            let chars: Vec<char> = evaluated.chars().collect();
            let destinations: Vec<usize> = program_data.local_symbols.get(&chars[0].to_string()).unwrap().to_vec();
            if chars[1] == 'F' {
                *(destinations.iter().filter(|d| **d > index).min().unwrap()) as i16
            }
            else if chars[1] == 'B' {
                *(destinations.iter().filter(|d| **d < index).max().unwrap()) as i16
            }
            else {
                panic!("Invalid local symbol");
            }
        }
        else {
            evaluated.parse::<i16>()
                .unwrap_or_else(|_| *program_data.label_table.get(&evaluated).expect(&format!("Unknown address {}", evaluated)) as i16)

        }
    };
    arch::HalfWord::from_value(val)
}

fn _evaluate(text: &str, index: usize, program_data: &ProgramData) -> String {
    if text == "*" {
        index.to_string()
    }
    else if text.contains('+') {
        let split: Vec<&str> = text.splitn(2, '+').collect();
        (_evaluate(split[0], index, program_data).parse::<i32>().expect(&format!("Invalid Digit: {}", split[0])) +
         _evaluate(split[1], index, program_data).parse::<i32>().expect(&format!("Invalid Digit: {}", split[1]))).to_string()
    }
    else if text.contains('-') && !text.starts_with('-') {
        let split: Vec<&str> = text.splitn(2, '-').collect();
        (_evaluate(split[0], index, program_data).parse::<i32>().unwrap() -
         _evaluate(split[1], index, program_data).parse::<i32>().unwrap()).to_string()
    }
    else if text.contains('/') {
        let split: Vec<&str> = text.splitn(2, '/').collect();
        (_evaluate(split[0], index, program_data).parse::<i32>().unwrap() /
         _evaluate(split[1], index, program_data).parse::<i32>().unwrap()).to_string()
    }
    else if text.contains('*') {
        let split: Vec<&str> = text.splitn(2, '*').collect();
        (_evaluate(split[0], index, program_data).parse::<i32>().unwrap() *
         _evaluate(split[1], index, program_data).parse::<i32>().unwrap()).to_string()

    }
    else {
        if program_data.label_table.contains_key(text) {
            program_data.label_table.get(text).unwrap().to_string()
        }
        else if program_data.symbol_table.contains_key(text) {
            program_data.symbol_table.get(text).unwrap().to_string()
        }
        else{
            String::from(text)
        }
    }
}

fn _get_index(spl: &Vec<&str>) -> u8 {
    if spl.len() > 1 {
        if spl[1].starts_with("(") {
            // this is the modifier field
            0
        }
        else {
            let index_split: Vec<&str> = spl[1].split("(").collect();
            index_split[0].parse::<u8>().expect(&format!("Invalid digit: {}", index_split[0]))
        }
    }
    else {
        0
    }
}

fn _get_modifier(op: &str, spl: Vec<&str>, program_data: &ProgramData) -> u8 {
   let default = match op {
        "HLT" => 2,
        "NUM" => 0,
        "CHAR" => 1,
        "SLA" => 0,
        "SRA" => 1,
        "SLAX" => 2,
        "SRAX" => 3,
        "SLC" => 4,
        "SRC" => 5,
        "MOVE" => 1,
        "STJ" => 2,
        "JBUS" | "IOC" | "IN" | "OUT" | "JRED" => 0,
        "JMP" => 0,
        "JSJ" => 1,
        "JOV" => 2,
        "JNOV" => 3,
        "JL" => 4,
        "JE" => 5,
        "JG" => 6,
        "JGE" => 7,
        "JNE" => 8,
        "JLE" => 9,
        "JAN" | "J1N" | "J2N" | "J3N" | "J4N" | "J5N" | "J6N" | "JXN" => 0,
        "JAZ" | "J1Z" | "J2Z" | "J3Z" | "J4Z" | "J5Z" | "J6Z" | "JXZ" => 1,
        "JAP" | "J1P" | "J2P" | "J3P" | "J4P" | "J5P" | "J6P" | "JXP" => 2,
        "JANN" | "J1NN" | "J2NN" | "J3NN" | "J4NN" | "J5NN" | "J6NN" | "JXNN" => 3,
        "JANZ" | "J1NZ" | "J2NZ" | "J3NZ" | "J4NZ" | "J5NZ" | "J6NZ" | "JXNZ" => 4,
        "JANP" | "J1NP" | "J2NP" | "J3NP" | "J4NP" | "J5NP" | "J6NP" | "JXNP" => 5,
        "INCA" | "INC1" | "INC2" | "INC3" | "INC4" | "INC5" | "INC6" | "INCX" => 0,
        "DECA" | "DEC1" | "DEC2" | "DEC3" | "DEC4" | "DEC5" | "DEC6" | "DECX" => 1,
        "ENTA" | "ENT1" | "ENT2" | "ENT3" | "ENT4" | "ENT5" | "ENT6" | "ENTX" => 2,
        "ENNA" | "ENN1" | "ENN2" | "ENN3" | "ENN4" | "ENN5" | "ENN6" | "ENNX" => 3,
        _ => 5
    };
    _parse_modifier(spl, default, program_data)
}

fn _parse_modifier(spl: Vec<&str>, default: u8, program_data: &ProgramData) -> u8 {
    if spl.len() > 1 {
        if spl[1].starts_with("(") {
            let modifier: Vec<&str> = spl[1].strip_prefix('(').unwrap().strip_suffix(')').unwrap().split(':').collect();
            modifier[0].parse::<u8>().unwrap() * 8 + modifier[1].parse::<u8>().unwrap()
        }
        else {
            let modifier_split: Vec<&str> = spl[1].split('(').collect();
            if modifier_split.len() > 1 {
                let modifier: Vec<&str> = modifier_split[1].strip_suffix(')').unwrap().split(':').collect();
                if modifier.len() == 2 {
                    modifier[0].parse::<u8>().unwrap() * 8 + modifier[1].parse::<u8>().unwrap()
                }
                else {
                    match program_data.symbol_table.get(modifier[0]) {
                        Some(val) => *val as u8,
                        None => modifier[0].parse::<u8>().unwrap()
                    }
                }
            }
            else
            {
                default
            }
        }

    }
    else {
        let modifier_split: Vec<&str> = spl[0].split('(').collect();
        if modifier_split.len() > 1 {
            let modifier: Vec<&str> = modifier_split[1].strip_suffix(')').unwrap().split(':').collect();
            if modifier.len() == 2 {
                modifier[0].parse::<u8>().unwrap() * 8 + modifier[1].parse::<u8>().unwrap()
            }
            else {
                match program_data.symbol_table.get(modifier[0]) {
                    Some(val) => *val as u8,
                    None => modifier[0].parse::<u8>().unwrap()
                }
            }
        }
        else
        {
            default
        }
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize(" HLT"), ("", "HLT", "")); // no label
        assert_eq!(tokenize("    HLT"), ("", "HLT", "")); // no label extra spaces
        assert_eq!(tokenize("X HLT"), ("X", "HLT", "")); // label
        assert_eq!(tokenize("X EQU  1000"), ("X", "EQU", "1000")); // label
        assert_eq!(tokenize("LABEL  HLT"), ("LABEL", "HLT", "")); // extra spaces
        assert_eq!(tokenize("LABEL  HLT"), ("LABEL", "HLT", "")); // extra spaces
        assert_eq!(tokenize("LABEL  ADD 240,1(2:5)"), ("LABEL", "ADD", "240,1(2:5)")); // address
        assert_eq!(tokenize("LABEL  ADD 240,1(2:5)"), ("LABEL", "ADD", "240,1(2:5)")); // extra remarks
    }

    #[test]
    fn test_parse_address_string() {
        assert_eq!(parse_address_string("ADD", "240,1(2:5)",0,  &ProgramData::new()), (arch::HalfWord::from_value(240), 1, 21));
        assert_eq!(parse_address_string("ADD", "240,1(15)", 0, &ProgramData::new()), (arch::HalfWord::from_value(240), 1, 15));
        assert_eq!(parse_address_string("ADD", "240,1", 0, &ProgramData::new()), (arch::HalfWord::from_value(240), 1, 5));
        assert_eq!(parse_address_string("ADD", "240", 0, &ProgramData::new()), (arch::HalfWord::from_value(240), 0, 5));
        assert_eq!(parse_address_string("ADD", "240(2:5)", 0, &ProgramData::new()), (arch::HalfWord::from_value(240), 0, 21));
        assert_eq!(parse_address_string("ADD", "240(15)", 0, &ProgramData::new()), (arch::HalfWord::from_value(240), 0, 15));
    }

    #[test]
    fn test_parse_address_string_with_symbol() {
        let mut program_data = ProgramData::new();
        program_data.symbol_table.insert("X".to_string(), 1000);
        program_data.symbol_table.insert("PRINTER".to_string(), 18);
        assert_eq!(parse_address_string("ADD", "X,1(2:5)", 0, &program_data), (arch::HalfWord::from_value(1000), 1, 21));
        assert_eq!(parse_address_string("ADD", "X+1,1(2:5)", 0, &program_data), (arch::HalfWord::from_value(1001), 1, 21));
        assert_eq!(parse_address_string("OUT", "X,1(PRINTER)", 0, &program_data), (arch::HalfWord::from_value(1000), 1, 18));
    }

    #[test]
    fn test_parse_address_string_with_asterisk() {
        let mut program_data = ProgramData::new();
        assert_eq!(parse_address_string("JMP", "*", 150, &program_data), (arch::HalfWord::from_value(150), 0, 0));
        assert_eq!(parse_address_string("JMP", "*+3", 150, &program_data), (arch::HalfWord::from_value(153), 0, 0));
        assert_eq!(parse_address_string("JMP", "*-3", 150, &program_data), (arch::HalfWord::from_value(147), 0, 0));
        assert_eq!(parse_address_string("JMP", "*+3", 250, &program_data), (arch::HalfWord::from_value(253), 0, 0));
    }

    #[test]
    fn test_parse_address_string_with_label() {
        let mut program_data = ProgramData::new();
        program_data.start_location = Some(150);
        program_data.label_table.insert("LOC".to_string(), 200);
        assert_eq!(parse_address_string("JMP", "LOC", 100, &program_data), (arch::HalfWord::from_value(200), 0, 0));
    }

    #[test]
    fn test_to_instruction() {
        assert_eq!(to_instruction(" HLT", 0, &ProgramData::new()), arch::Word::from_values(true, 0,0,0,2,5));
        assert_eq!(to_instruction(" NOP", 0, &ProgramData::new()), arch::Word::from_values(true, 0,0,0,0,0));
        assert_eq!(to_instruction(" ADD 100,1(2:5)", 0, &ProgramData::new()), arch::Word::from_values(true, 1,36,1,21,1));
        assert_eq!(to_instruction(" ADD 100", 0, &ProgramData::new()), arch::Word::from_values(true, 1,36,0,5,1));
        assert_eq!(to_instruction(" SUB 100,3(2:5)", 0, &ProgramData::new()), arch::Word::from_values(true, 1,36,3,21,2));
        assert_eq!(to_instruction(" MUL -12,3(3:5)", 0, &ProgramData::new()), arch::Word::from_values(false, 0,12,3,29,3));
        assert_eq!(to_instruction(" DIV -12,3(3:5)", 0, &ProgramData::new()), arch::Word::from_values(false, 0,12,3,29,4));
        assert_eq!(to_instruction(" NUM 5", 0, &ProgramData::new()), arch::Word::from_values(true, 0,5,0,0,5));
        assert_eq!(to_instruction(" CHAR 5", 0, &ProgramData::new()), arch::Word::from_values(true, 0,5,0,1,5));
        assert_eq!(to_instruction(" SLA 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,6));
        assert_eq!(to_instruction(" SRA 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,6));
        assert_eq!(to_instruction(" SLAX 2,1", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,1,2,6));
        assert_eq!(to_instruction(" SRAX 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,6));
        assert_eq!(to_instruction(" SLC 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,6));
        assert_eq!(to_instruction(" SRC 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,6));
        assert_eq!(to_instruction(" MOVE 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,7));
        assert_eq!(to_instruction(" LDA 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,8));
        assert_eq!(to_instruction(" LD1 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,9));
        assert_eq!(to_instruction(" LD2 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,10));
        assert_eq!(to_instruction(" LD3 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,11));
        assert_eq!(to_instruction(" LD4 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,12));
        assert_eq!(to_instruction(" LD5 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,13));
        assert_eq!(to_instruction(" LD6 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,14));
        assert_eq!(to_instruction(" LDX 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,15));
        assert_eq!(to_instruction(" LDAN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,16));
        assert_eq!(to_instruction(" LD1N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,17));
        assert_eq!(to_instruction(" LD2N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,18));
        assert_eq!(to_instruction(" LD3N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,19));
        assert_eq!(to_instruction(" LD4N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,20));
        assert_eq!(to_instruction(" LD5N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,21));
        assert_eq!(to_instruction(" LD6N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,22));
        assert_eq!(to_instruction(" LDXN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,23));
        assert_eq!(to_instruction(" STA 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,24));
        assert_eq!(to_instruction(" ST1 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,25));
        assert_eq!(to_instruction(" ST2 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,26));
        assert_eq!(to_instruction(" ST3 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,27));
        assert_eq!(to_instruction(" ST4 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,28));
        assert_eq!(to_instruction(" ST5 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,29));
        assert_eq!(to_instruction(" ST6 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,30));
        assert_eq!(to_instruction(" STX 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,31));
        assert_eq!(to_instruction(" STJ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,32));
        assert_eq!(to_instruction(" STZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,33));
        assert_eq!(to_instruction(" JBUS 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,34));
        assert_eq!(to_instruction(" IOC 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,35));
        assert_eq!(to_instruction(" IN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,36));
        assert_eq!(to_instruction(" OUT 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,37));
        assert_eq!(to_instruction(" JRED 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,38));
        assert_eq!(to_instruction(" JMP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,39));
        assert_eq!(to_instruction(" JSJ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,39));
        assert_eq!(to_instruction(" JOV 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,39));
        assert_eq!(to_instruction(" JNOV 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,39));
        assert_eq!(to_instruction(" JL 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,39));
        assert_eq!(to_instruction(" JE 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,39));
        assert_eq!(to_instruction(" JG 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,6,39));
        assert_eq!(to_instruction(" JGE 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,7,39));
        assert_eq!(to_instruction(" JNE 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,8,39));
        assert_eq!(to_instruction(" JLE 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,9,39));
        assert_eq!(to_instruction(" JAN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,40));
        assert_eq!(to_instruction(" JAZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,40));
        assert_eq!(to_instruction(" JAP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,40));
        assert_eq!(to_instruction(" JANN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,40));
        assert_eq!(to_instruction(" JANZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,40));
        assert_eq!(to_instruction(" JANP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,40));
        assert_eq!(to_instruction(" J1N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,41));
        assert_eq!(to_instruction(" J1Z 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,41));
        assert_eq!(to_instruction(" J1P 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,41));
        assert_eq!(to_instruction(" J1NN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,41));
        assert_eq!(to_instruction(" J1NZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,41));
        assert_eq!(to_instruction(" J1NP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,41));
        assert_eq!(to_instruction(" J2N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,42));
        assert_eq!(to_instruction(" J2Z 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,42));
        assert_eq!(to_instruction(" J2P 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,42));
        assert_eq!(to_instruction(" J2NN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,42));
        assert_eq!(to_instruction(" J2NZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,42));
        assert_eq!(to_instruction(" J2NP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,42));
        assert_eq!(to_instruction(" J3N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,43));
        assert_eq!(to_instruction(" J3Z 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,43));
        assert_eq!(to_instruction(" J3P 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,43));
        assert_eq!(to_instruction(" J3NN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,43));
        assert_eq!(to_instruction(" J3NZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,43));
        assert_eq!(to_instruction(" J3NP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,43));
        assert_eq!(to_instruction(" J4N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,44));
        assert_eq!(to_instruction(" J4Z 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,44));
        assert_eq!(to_instruction(" J4P 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,44));
        assert_eq!(to_instruction(" J4NN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,44));
        assert_eq!(to_instruction(" J4NZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,44));
        assert_eq!(to_instruction(" J4NP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,44));
        assert_eq!(to_instruction(" J5N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,45));
        assert_eq!(to_instruction(" J5Z 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,45));
        assert_eq!(to_instruction(" J5P 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,45));
        assert_eq!(to_instruction(" J5NN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,45));
        assert_eq!(to_instruction(" J5NZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,45));
        assert_eq!(to_instruction(" J5NP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,45));
        assert_eq!(to_instruction(" J6N 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,46));
        assert_eq!(to_instruction(" J6Z 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,46));
        assert_eq!(to_instruction(" J6P 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,46));
        assert_eq!(to_instruction(" J6NN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,46));
        assert_eq!(to_instruction(" J6NZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,46));
        assert_eq!(to_instruction(" J6NP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,46));
        assert_eq!(to_instruction(" JXN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,47));
        assert_eq!(to_instruction(" JXZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,47));
        assert_eq!(to_instruction(" JXP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,47));
        assert_eq!(to_instruction(" JXNN 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,47));
        assert_eq!(to_instruction(" JXNZ 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,4,47));
        assert_eq!(to_instruction(" JXNP 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,47));
        assert_eq!(to_instruction(" INCA 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,48));
        assert_eq!(to_instruction(" DECA 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,48));
        assert_eq!(to_instruction(" ENTA 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,48));
        assert_eq!(to_instruction(" ENNA 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,48));
        assert_eq!(to_instruction(" INC1 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,49));
        assert_eq!(to_instruction(" DEC1 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,49));
        assert_eq!(to_instruction(" ENT1 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,49));
        assert_eq!(to_instruction(" ENN1 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,49));
        assert_eq!(to_instruction(" INC2 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,50));
        assert_eq!(to_instruction(" DEC2 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,50));
        assert_eq!(to_instruction(" ENT2 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,50));
        assert_eq!(to_instruction(" ENN2 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,50));
        assert_eq!(to_instruction(" INC3 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,51));
        assert_eq!(to_instruction(" DEC3 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,51));
        assert_eq!(to_instruction(" ENT3 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,51));
        assert_eq!(to_instruction(" ENN3 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,51));
        assert_eq!(to_instruction(" INC4 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,52));
        assert_eq!(to_instruction(" DEC4 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,52));
        assert_eq!(to_instruction(" ENT4 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,52));
        assert_eq!(to_instruction(" ENN4 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,52));
        assert_eq!(to_instruction(" INC5 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,53));
        assert_eq!(to_instruction(" DEC5 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,53));
        assert_eq!(to_instruction(" ENT5 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,53));
        assert_eq!(to_instruction(" ENN5 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,53));
        assert_eq!(to_instruction(" INC6 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,54));
        assert_eq!(to_instruction(" DEC6 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,54));
        assert_eq!(to_instruction(" ENT6 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,54));
        assert_eq!(to_instruction(" ENN6 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,54));
        assert_eq!(to_instruction(" INCX 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,0,55));
        assert_eq!(to_instruction(" DECX 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,1,55));
        assert_eq!(to_instruction(" ENTX 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,2,55));
        assert_eq!(to_instruction(" ENNX 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,3,55));
        assert_eq!(to_instruction(" CMPA 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,56));
        assert_eq!(to_instruction(" CMP1 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,57));
        assert_eq!(to_instruction(" CMP2 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,58));
        assert_eq!(to_instruction(" CMP3 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,59));
        assert_eq!(to_instruction(" CMP4 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,60));
        assert_eq!(to_instruction(" CMP5 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,61));
        assert_eq!(to_instruction(" CMP6 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,62));
        assert_eq!(to_instruction(" CMPX 2", 0, &ProgramData::new()), arch::Word::from_values(true, 0,2,0,5,63));
    }

    #[test]
    fn test_to_instruction_constant() {
        let mut program_data = ProgramData::new();
        assert_eq!(to_instruction(" CON 2", 0, &program_data), arch::Word::from_values(true, 0,0,0,0,2));

        program_data.symbol_table.insert("X".to_string(), 125);
        assert_eq!(to_instruction(" CON X+3", 0, &program_data), arch::Word::from_values(true, 0,0,0,2,0));

        assert_eq!(to_instruction(" CON -1", 0, &program_data), arch::Word::from_values(false, 0,0,0,0,1));

    }

    #[test]
    fn test_to_alphabetic_constant() {
        let program_data = ProgramData::new();
        assert_eq!(to_instruction(" ALF ABCDE", 0, &program_data), arch::Word::from_values(true, 1,2,3,4,5));
        assert_eq!(to_instruction(" ALF  DEFGH", 0, &program_data), arch::Word::from_values(true, 4,5,6,7,8));
        assert_eq!(to_instruction(" ALF   EFGH", 0, &program_data), arch::Word::from_values(true, 0,5,6,7,8));
    }

    #[test]
    fn test_extra_remarks() {
        assert_eq!(to_instruction("LABEL ADD 100,1(2:5) * Extra remarks", 0, &ProgramData::new()), arch::Word::from_values(true, 1, 36, 1, 21, 1));
    }

}
