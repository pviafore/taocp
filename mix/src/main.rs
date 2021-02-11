mod arch;
mod cards;
mod chartable;
mod computer;
mod instructions;
mod io;
mod timing;

use clap::Clap;
use std::fs;
use std::io::Write;

#[derive(Clap)]
#[clap(version = "1.0", author = "Pat V <patviafore@gmail.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    CreateCardpack(CreateCardpack),
    Run(Run)
}

#[derive(Clap)]
struct CreateCardpack {
    cardpack_name: String,
    program_file: String,
    start_location: i16
}

#[derive(Clap)]
struct Run {
    cardpack_name: String,
    #[clap(long, short)]
    trace: bool,
    #[clap(long, short='x')]
    timing: bool
}

fn main() {

    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::CreateCardpack(cmd) => { create_cardpack(cmd) }
        SubCommand::Run(cmd) => { run(cmd) }
    }
}

fn create_cardpack(cmd: CreateCardpack) {
    // this is derived from bootloader.mixal
    let loading_card_1 = " O O6 A O4    I 2 O6 C O4 3 EH A  F F CF 0  E = EU 3 IH H BB $ EU = EJ  CA. 5A-H\n";
    let loading_card_2 = " U BB  C U = EH F BA = EU 4AEH 5AEN  ABG S  E  CLU $ EH F BB $ EU L B. B  9     \n";
    let transfer_card  = format!("+TRANS0{:04}", cmd.start_location);
    let filename = format!("{}", cmd.cardpack_name);
    let mut file = std::fs::File::create(filename).expect("create failed");
    file.write_all(loading_card_1.as_bytes()).unwrap();
    file.write_all(loading_card_2.as_bytes()).unwrap();
    let contents = fs::read(cmd.program_file).unwrap();
    let punch_cards = cards::convert_to_punch_cards(contents, cmd.start_location);
    file.write_all(&punch_cards.as_bytes()).unwrap();
    file.write_all(transfer_card.as_bytes()).unwrap();

}

fn run(cmd: Run) {
    use std::str;
    let mut comp = computer::Computer::new();
    if cmd.trace {
        comp.turn_tracing_on()
    }
    let bytes = fs::read(cmd.cardpack_name).unwrap();
    let contents: Vec<&str> = str::from_utf8(&bytes).unwrap().split("\n").collect();
    comp.add_card(chartable::translate(contents[0]));
    comp.add_card(chartable::translate(contents[1]));
    for i in 2..(contents.len()) {
        comp.add_card(cards::translate_data_card(contents[i]))
    }
    comp.run_from_cards();
    if cmd.timing {
        println!("Time to run: {:?}", comp.get_time_to_run())
    }
}
