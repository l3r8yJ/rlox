use anyhow::Ok;
use anyhow::Result;

#[derive(Debug)]
pub struct Lex {
    pub had_error: bool,
}

impl Lex {
    pub fn new() -> Lex {
        Lex { had_error: false }
    }
}

#[test]
fn should_create_empty_lex() -> Result<()> {
    let lex = Lex::new();
    assert_eq!(
        lex.had_error, false,
        "New lex should be without an errors, but was {:?}",
        lex
    );
    Ok(())
}
