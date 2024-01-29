use std::fs::read_to_string;
use std::io::stdin;

use crate::lex::lex::Lex;
use crate::token::token::Token;

impl Lex {
    pub fn run_file(&self, file: Option<&String>) {
        let program = match read_to_string(file.unwrap()) {
            Ok(str) => str,
            Err(e) => {
                println!("Can't execute file: {e}");
                std::process::exit(1);
            }
        };
        self.run(&*program);

        if self.had_error {
            std::process::exit(65);
        };
    }

    pub fn run_prompt(&mut self) {
        loop {
            let mut line = String::new();
            print!("> ");
            let i = match stdin().read_line(&mut line) {
                Ok(command) => command,
                Err(e) => {
                    println!("Can't execute prompt: {e}");
                    std::process::exit(1);
                }
            };
            if i == 0 {
                break;
            }
            self.run(&*line);
            self.had_error = false
        }
    }

    pub fn run(&self, program: &str) {
        let tokens = self.scan_tokens(program);
        tokens.iter().for_each(|t| println!("{:?}", t))
    }

    fn scan_tokens(&self, _source: &str) -> Vec<Token> {
        todo!()
    }
}
