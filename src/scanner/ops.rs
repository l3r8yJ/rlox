use std::string::String;

use anyhow::Result;

use TokenType::{
    Bang, BangEqual, Comma, Dot, Equal, EqualEqual, Greater, GreaterEqual, LeftBrace, LeftParen,
    Less, LessEqual, Minus, Plus, RightBrace, RightParen, Semicolon, Slash, Star, EOF,
};

use crate::scanner::scanner::Scanner;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

impl Scanner {
    pub fn scan_tokens(&mut self) -> &mut [Token] {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens
            .push(Token::new(EOF, "".to_string(), "".to_string(), self.line));
        self.tokens.as_mut_slice()
    }

    pub(crate) fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();
        let mut with_equal_or_not = |left: TokenType, right: TokenType| -> TokenType {
            let contains_equality: bool = self.contains('=');
            return if contains_equality { left } else { right };
        };
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let token = with_equal_or_not(BangEqual, Bang);
                self.add_token(token);
            }
            '=' => {
                let token = with_equal_or_not(EqualEqual, Equal);
                self.add_token(token);
            }
            '<' => {
                let token = with_equal_or_not(LessEqual, Less);
                self.add_token(token);
            }
            '>' => {
                let token = with_equal_or_not(GreaterEqual, Greater);
                self.add_token(token);
            }
            '/' => {
                if self.contains('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(Slash)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string_advance(),
            _ => {
                todo!("Add logging with err")
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_liter(token_type, "".to_string());
    }

    fn add_token_liter(&mut self, token_type: TokenType, literal: String) {
        let text: &str = &self.source.as_str()[self.start..self.current];
        self.tokens
            .push(Token::new(token_type, text.to_string(), literal, self.line))
    }

    fn contains(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let current = match self.source.chars().nth(self.current) {
            Some(c) => c,
            None => panic!("Ann error occurred, current char not found"),
        };
        if current != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        return if self.is_at_end() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        };
    }

    fn string_advance(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.is_at_end() {
            panic!("Unterminated string");
        }
        self.advance();
        let string_value = &self.source.as_str()[(self.start + 1)..(self.current - 1)];
        self.add_token_liter(TokenType::String, string_value.to_string());
    }
}

#[test]
fn should_get_string_value() -> Result<()> {
    let expected = "This is just a text inside of string";
    let source = "{\"This is just a text inside of string\"}";
    let expected_len = 4;
    let string_position = 1;
    let mut scanner: Scanner = Scanner::new(source.to_string(), vec![]);
    scanner.scan_tokens();
    assert_eq!(
        scanner.tokens.len(),
        expected_len,
        "{:?} should contain each of {source}",
        scanner.tokens
    );
    assert_eq!(
        scanner.tokens.get(string_position).unwrap().literal,
        expected
    );
    Ok(())
}

#[test]
fn should_ignore_whitespaces() -> Result<()> {
    let expected_len = 1;
    let source: &str = "\n\r\n\t ";
    let mut scanner: Scanner = Scanner::new(source.to_string(), vec![]);
    scanner.scan_tokens();
    assert_eq!(
        scanner.tokens.len(),
        expected_len,
        "{:?} should contain only EOF",
        scanner.tokens
    );
    Ok(())
}

#[test]
fn should_add_valid_tokens() -> Result<()> {
    let expected_len = 22;
    let source: &str = "{} () + - / * < > <= >= != == = != ! ; * , .";
    let mut scanner: Scanner = Scanner::new(source.to_string(), vec![]);
    scanner.scan_tokens();
    assert_eq!(
        scanner.tokens.len(),
        expected_len,
        "{:?} should contain each of {source}",
        scanner.tokens
    );
    Ok(())
}

#[test]
fn should_not_be_at_the_end() -> Result<()> {
    let scanner = Scanner::new("aaa".to_string(), vec![]);
    assert!(!scanner.is_at_end());
    Ok(())
}

#[test]
fn should_be_at_the_end() -> Result<()> {
    let mut scanner = Scanner::new("aaa".to_string(), vec![]);
    scanner.current = 5;
    assert!(scanner.is_at_end());
    Ok(())
}
