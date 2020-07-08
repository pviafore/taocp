mod arch;
mod computer;
mod instructions;


fn main() {
    let word = arch::Word::new();
    println!("{}", word.is_positive);
}
