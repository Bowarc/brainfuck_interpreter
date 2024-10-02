#[macro_use]
extern crate log;

mod instruction;
mod interpreter;

fn main() {
    logger::init(logger::LoggerConfig::default(), None);

    if let Some(seq) = interpreter::init() {
        let bf_seq = seq
            .iter()
            .map(|inst| char::from(inst.clone()))
            .collect::<Vec<char>>()
            .iter()
            .collect::<String>();

        println!("Brainfuck sequence: {bf_seq}\n");
        println!("Instructions: {seq:?}\n");

        interpreter::run(seq);
    } else {
        debug!("No file supplied, supply a brainfuck file to run the interpreter");
    }
}
