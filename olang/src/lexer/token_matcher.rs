use crate::lexer::token::Token;

pub trait TokenMatcher<'a> {
    fn is_match(&self, chars: &Vec<&'a char>) -> bool;
    fn create_token(&self, chars: &Vec<&'a char>, localisation: TokenLocalisation) -> Token;
}

pub struct TokenLocalisation {
    pub line: usize,
    pub line_start: usize,
    pub line_end: usize,
}
