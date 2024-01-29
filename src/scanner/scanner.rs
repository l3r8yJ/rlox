use crate::token::token::Token;

#[derive(Debug)]
pub struct Scanner {
    pub(crate) source: String,
    pub(crate) tokens: Vec<Token>,
    pub(crate) start: usize,
    pub(crate) current: usize,
    pub(crate) line: usize,
}

impl Scanner {
    pub fn new(source: String, tokens: Vec<Token>) -> Scanner {
        Scanner {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 0,
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
