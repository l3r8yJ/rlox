use crate::token::token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: String,
    pub(crate) line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}

#[test]
fn should_create_empty_token() -> anyhow::Result<()> {
    let token = Token::new(TokenType::And, "aaa".into(), "bbb".into(), 10);
    assert_eq!(token.token_type, TokenType::And);
    assert_eq!(token.lexeme, "aaa");
    assert_eq!(token.literal, "bbb");
    assert_eq!(token.line, 10);
    anyhow::Ok(())
}
