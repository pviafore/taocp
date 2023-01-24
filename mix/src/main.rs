mod arch;
mod assembler;
mod cards;
mod chartable;
mod computer;
mod instructions;
mod io;
mod timing;

use clap::Parser;
use std::fs::{self, File};
use std::io::{BufRead, Write};

#[derive(clap::Parser)]
#[command(version = "1.0", author = "Pat V <patviafore@gmail.com>")]
struct Opts {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    CreateCardpack(CreateCardpack),
    Run(Run),
    Assemble(Assemble)
}

#[derive(Parser)]
struct CreateCardpack {
    cardpack_name: String,
    program_file: String,
    start_location: i16
}

#[derive(Parser)]
struct Run {
    program_name: String,
    #[arg(long, short, required=false, default_value="mixal")]
    from: String,
    #[arg(long, short, required=false, default_value="-1")]
    start: i16,
    #[arg(long, short='T', required=false, default_value="invalid")]
    paper_tape_file: String,
    #[arg(long, required=false, default_value="invalid")]
    tape_0_file: String,
    #[arg(long, required=false, default_value="invalid")]
    tape_1_file: String,
    #[arg(long, short='D', required=false, default_value="invalid")]
    data_cards_file: String,
    #[arg(long, short='p')]
    as_program_cards: bool,
    #[arg(long, short='x')]
    trace: bool,
    #[arg(long, short='d')]
    debugger: bool,
    #[arg(long, short)]
    timing: bool,
    #[arg(long, short)]
    verbose: bool
}

#[derive(Parser)]
struct Assemble {
    input_filename: String,
    output_filename: String
}

fn main() {

    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::CreateCardpack(cmd) => { create_cardpack(cmd) }
        SubCommand::Run(cmd) => { run(cmd) }
        SubCommand::Assemble(cmd) => { assemble(cmd)}
    }
}

fn create_cardpack(cmd: CreateCardpack) {
    // this is derived from bootloader.mixal
    let filename = format!("{}", cmd.cardpack_name);
    let mut file = std::fs::File::create(filename).expect("create failed");
    let contents = fs::read(cmd.program_file).unwrap();
    let cards = convert_mix_to_cardpack(contents, cmd.start_location as usize, cmd.start_location as usize);
    file.write_all(cards.as_bytes()).unwrap();
}

fn convert_mix_to_cardpack(bytes: Vec<u8>, load_address: usize, start_location: usize) -> String {
    let loading_card_1 = " O O6 A O4    I 2 O6 D O4 3 EH A  F F CF 0  E = EU 3 IH H BB $ EU = EJ  CA. 5A-H\n";
    let loading_card_2 = " U BB  C U = EH F BA = EU 4AEH 5AEN  ABG S  E  CLU $ EH F BB $ EU L B. B  9     \n";
    let transfer_card  = format!("+TRANS0{:04}", start_location);
    let punch_cards = cards::convert_to_punch_cards(bytes, load_address as usize);
    format!("{}{}{}{}", loading_card_1, loading_card_2, punch_cards, transfer_card)
}

fn load_cards_into_computer(cardpack_bytes: Vec<u8>) -> computer::Computer{
    use std::str;
    let mut comp = computer::Computer::new();
    let contents: Vec<&str> = str::from_utf8(&cardpack_bytes).unwrap().split("\n").collect();
    comp.add_card(chartable::translate(contents[0]));
    comp.add_card(chartable::translate(contents[1]));
    for i in 2..(contents.len()) {
        comp.add_card(cards::translate_program_card(contents[i]))
    }
    comp
}

fn get_computer(program_filename: String, from: String, start: i16, verbose: bool) -> computer::Computer {
    if from == "cardpack" {
        let bytes = fs::read(program_filename).unwrap();
        if start != -1 {
            println!("Ignoring start address {:?} for running from cardpack", start)
        }
        load_cards_into_computer(bytes)
    }
    else if from == "mix" {
        let bytes = fs::read(program_filename).unwrap();
        if start == -1 {
            panic!("Must provide start address for running mix")
        }
        let cardpack = convert_mix_to_cardpack(bytes, start as usize, start as usize);
        load_cards_into_computer(cardpack.as_bytes().to_vec())
    }
    else if from == "mixal"{
        let file = File::open(program_filename).unwrap();
        let lines: Vec<String> = std::io::BufReader::new(file).lines().map(|x| x.unwrap()).collect();
        let (assembled, load_address, start_location, label_map) = assembler::assemble(lines, verbose);
        let program_start = match start_location {
            Some(loc) => if start != -1 {
                panic!("Program start already lives inside program")
            }
            else {
                loc
            }
            None => {
                if start == -1 {
                    panic!("Must provide start address for running mixal")
                }
                else {
                    start as usize
                }
            }
        };
        let cardpack = convert_mix_to_cardpack(assembled, load_address, program_start);
        let mut computer = load_cards_into_computer(cardpack.as_bytes().to_vec());

        computer.set_label_map(label_map);
        computer
    }
    else {
        panic!("Can only read from cardpacks, mix, or mixal files");
    }

}

fn run(cmd: Run) {
    let mut comp = get_computer(cmd.program_name, cmd.from, cmd.start, cmd.verbose);
    if cmd.data_cards_file != "invalid" {
        use std::str;
        let bytes = fs::read(cmd.data_cards_file).unwrap();
        let contents: Vec<&str> = str::from_utf8(&bytes).unwrap().split("\n").collect();
        if cmd.as_program_cards {
            comp.add_card(chartable::translate(contents[0]));
            comp.add_card(chartable::translate(contents[1]));
            for i in 2..(contents.len()) {
                comp.add_card(cards::translate_program_card(contents[i]));
            }
        }
        else {
            for i in 0..(contents.len()) {
                comp.add_card(chartable::translate(contents[i]));
            }
        }
    }
    if cmd.trace {
        comp.turn_tracing_on()
    }
    if cmd.paper_tape_file != "invalid" {
        let file = File::open(cmd.paper_tape_file).unwrap();
        let mut line = String::new();
        std::io::BufReader::new(file).read_line(&mut line).unwrap();
        comp.load_paper_tape(chartable::translate(&line.strip_suffix('\n').unwrap()));
    }
    if cmd.tape_0_file != "invalid" {
        let file = File::open(cmd.tape_0_file).unwrap();
        let mut line = String::new();
        std::io::BufReader::new(file).read_line(&mut line).unwrap();
        comp.load_tape(0, chartable::translate(&line.strip_suffix('\n').unwrap_or(&line)));
    }
    if cmd.tape_1_file != "invalid" {
        let file = File::open(cmd.tape_1_file).unwrap();
        let mut line = String::new();
        std::io::BufReader::new(file).read_line(&mut line).unwrap();
        comp.load_tape(1, chartable::translate(&line.strip_suffix('\n').unwrap_or(&line)));
    }

    comp.load_card_into_memory();
    comp.run(cmd.debugger);
    if cmd.timing {
        println!("Time to run: {:?}", comp.get_time_to_run())
    }
}

fn assemble(cmd: Assemble) {
    let file = File::open(cmd.input_filename).unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file).lines().map(|x| x.unwrap()).collect();
    let (assembled, _load_address, _start_location, _) = assembler::assemble(lines, false);
    let mut output_file = std::fs::File::create(cmd.output_filename).expect("create failed");
    output_file.write_all(&assembled).unwrap();
}
