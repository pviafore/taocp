mod arch;
mod computer;
mod instructions;
mod io;
mod timing;

use std::env;


fn main() {
    let computer = computer::Computer::new();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => { println!("Need to pass in a command") },
        3 => { handle_command(args)},
        _ => { println!(" Too many commands")}
    }
}

fn handle_command(args: Vec<String>) {
    match &args[1][..] {
        "run" => {
            println!("run")
        }
        _ => { panic!("Invalid command")}
    }
}
