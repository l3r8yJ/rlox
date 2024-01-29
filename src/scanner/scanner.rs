use std::collections::HashMap;

use crate::token::token::Token;
use crate::token::token_type::TokenType;
use crate::token::token_type::TokenType::{
    And, Class, Else, False, For, Fun, If, Nil, Or, Print, Return, Super, This, True, Var, While,
};

#[derive(Debug)]
pub struct Scanner {
    pub(crate) source: String,
    pub(crate) tokens: Vec<Token>,
    pub(crate) start: usize,
    pub(crate) current: usize,
    pub(crate) line: usize,
    pub(crate) keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String, tokens: Vec<Token>) -> Scanner {
        Scanner {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 0,
            keywords: HashMap::from([
                (String::from("and"), And),
                (String::from("class"), Class),
                (String::from("else"), Else),
                (String::from("false"), False),
                (String::from("for"), For),
                (String::from("fun"), Fun),
                (String::from("if"), If),
                (String::from("nil"), Nil),
                (String::from("or"), Or),
                (String::from("print"), Print),
                (String::from("return"), Return),
                (String::from("super"), Super),
                (String::from("this"), This),
                (String::from("true"), True),
                (String::from("var"), Var),
                (String::from("while"), While),
            ]),
        }
    }
}

#[test]
fn should_create_scanner() -> anyhow::Result<()> {
    let scan = Scanner::new("meh".to_string(), vec![]);
    assert!(scan.tokens.is_empty(), "Should be empty {:?}", scan);
    assert_eq!(scan.source, "meh");
    anyhow::Ok(())
}
