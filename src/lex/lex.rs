#[derive(Debug)]
pub struct Lex {
    pub had_error: bool,
}

impl Lex {
    pub fn lox_error(&mut self, line: i32, message: &str) {
        self.report(line, message, "");
        self.had_error = true;
    }

    fn report(&self, line: i32, message: &str, place: &str) {
        eprintln!("[{line}] Error occurred {place} : {message}");
    }
}