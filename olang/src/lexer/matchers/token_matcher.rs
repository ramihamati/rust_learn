use crate::lexer::InputReader;
use crate::lexer::Token;

pub trait TokenMatcher<'a> {
    fn create(&self, reader: &mut InputReader) -> Option<Token>;
}

pub struct TokenLocalisation {
    pub line: usize,
    pub line_start: usize,
    pub line_end: usize,
}