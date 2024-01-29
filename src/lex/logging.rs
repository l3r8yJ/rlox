use crate::lex::lex::Lex;

impl Lex {
    pub fn lex_error(&mut self, line: usize, message: &str) {
        self.report(line, message, "");
        self.had_error = true;
    }

    fn report(&self, line: usize, message: &str, place: &str) {
        eprintln!("[{line}] Error occurred {place}: {message}");
    }
}

#[test]
fn should_change_error_state_after_err_logging() -> anyhow::Result<()> {
    let mut lex = Lex::new();
    lex.lex_error(0, "some error");
    assert_eq!(
        lex.had_error, true,
        "Error state should be true but was {:?}",
        lex
    );
    anyhow::Ok(())
}
