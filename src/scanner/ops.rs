use std::string::String;

use anyhow::Result;

use TokenType::{
    Bang, BangEqual, Comma, Dot, Equal, EqualEqual, Greater, GreaterEqual, LeftBrace, LeftParen,
    Less, LessEqual, Minus, Plus, RightBrace, RightParen, Semicolon, Slash, Star, EOF,
};

use crate::scanner::scanner::Scanner;
use crate::token::token::Token;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::{Identifier, Number};

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
                if Self::is_digit(c) {
                    self.number_advice();
                } else if Self::is_alpha(c) {
                    self.identifier_advice()
                } else {
                    panic!("An error")
                }
            }
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_literal_token(token_type, "".to_string());
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: String) {
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
            None => panic!("Current char not found"),
        };
        if current != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
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
        self.add_literal_token(TokenType::String, string_value.to_string());
    }

    fn number_advice(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && Self::is_digit(self.peek_next()) {
            self.advance();
            while Self::is_digit(self.peek()) {
                self.advance();
            }
        };
        let value = &self.source.as_str()[self.start..self.current];
        self.add_literal_token(Number, value.to_string());
    }

    fn identifier_advice(&mut self) {
        while Self::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = &self.source.as_str()[self.start..self.current];
        let token = self.keywords.get(text).unwrap_or(&Identifier).clone();
        self.add_token(token);
    }

    fn is_alpha_numeric(c: char) -> bool {
        return Self::is_alpha(c) || Self::is_digit(c);
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_digit(c: char) -> bool {
        return c >= '0' && c <= '9';
    }
}

#[test]
fn should_scan_identifiers_with_keywords() -> Result<()> {
    let expected_len = 13;
    let source = r#"
        while (true) {
            identifier = identifier + 1;
        }
    "#;
    let mut scanner = Scanner::new(source.to_string(), vec![]);
    scanner.scan_tokens();
    assert_eq!(
        scanner.tokens.len(),
        expected_len,
        "{:?} should have len {} but was {}",
        scanner.tokens,
        expected_len,
        scanner.tokens.len()
    );
    Ok(())
}

#[test]
fn should_scan_some_keywords() -> Result<()> {
    let expected_len = 5;
    let source = "class while if fun";
    let mut scanner = Scanner::new(source.to_string(), vec![]);
    scanner.scan_tokens();
    assert_eq!(
        scanner.tokens.len(),
        expected_len,
        "{:?} should have len {} but was {}",
        scanner.tokens,
        expected_len,
        scanner.tokens.len()
    );
    Ok(())
}

#[test]
fn should_read_floating_numbers() -> Result<()> {
    let source = "1.23 2.34 3.45 4.56";
    let expected_len = 5;
    let mut scanner = Scanner::new(source.to_string(), vec![]);
    scanner.scan_tokens();
    assert_eq!(
        scanner.tokens.len(),
        expected_len,
        "{:?} should have len {} but was {}",
        scanner.tokens,
        expected_len,
        scanner.tokens.len()
    );
    Ok(())
}

#[test]
fn should_read_int_numbers() -> Result<()> {
    let source = "1 2 3 4";
    let expected_len = 5;
    let mut scanner = Scanner::new(source.to_string(), vec![]);
    scanner.scan_tokens();
    assert_eq!(
        scanner.tokens.len(),
        expected_len,
        "{:?} should have len {} but was {}",
        scanner.tokens,
        expected_len,
        scanner.tokens.len()
    );
    Ok(())
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
