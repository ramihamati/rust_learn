use crate::lexer::token::Token;

pub trait TokenMatcher<'a> {
    fn is_match(&self, chars: &'a str) -> bool;
    fn create_token(&self, chars: &'a str, localisation: TokenLocalisation) -> Token;
}

pub struct TokenLocalisation {
    pub line: usize,
    pub line_start: usize,
    pub line_end: usize,
}
