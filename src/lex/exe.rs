use std::fs::File;
use std::io::{stdin, Read};

use crate::lex::lex::Lex;
use crate::scanner::scanner::Scanner;
use crate::token::token::Token;

impl Lex {
    pub fn run_file(&self, file: Option<&String>) {
        let mut file = File::open(file.unwrap()).expect("Should have been able to read the file");
        let mut buff: Vec<u8> = vec![];
        file.read_to_end(&mut buff).expect("Can't read file");
        let program = match std::str::from_utf8(buff.as_slice()) {
            Ok(text) => text,
            Err(e) => panic!("{:?}", e),
        };
        self.run(program);
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

    fn scan_tokens(&self, source: &str) -> Vec<Token> {
        let mut scanner = Scanner::new(source.to_string(), vec![]);
        scanner.scan_tokens().to_vec()
    }
}
