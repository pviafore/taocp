mod arch;
mod computer;
mod instructions;
mod io;
mod timing;


fn main() {
    let word = arch::Word::new();
    println!("{}", word.is_positive);
}
