use std::env;

use crate::lex::lex::Lex;

mod lex;
mod scanner;
mod token;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut lex: Lex = Lex::new();
    match args.len() {
        2 => lex.run_file(args.first()),
        1 => lex.run_prompt(),
        _ => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
    };
}
