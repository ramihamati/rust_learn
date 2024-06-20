use std::fmt::{Display, Formatter};
use crate::lexer::LiteralValue;
use crate::lexer::TokenType;

#[derive(Debug, Clone)]
pub enum TokenState{
    Ok,
    MissingEndToken
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal_value: Option<LiteralValue>,
    pub position: usize,
    pub line_number: usize,
    pub state: TokenState,
}

impl Token {
    pub fn to_string(self: &Self) -> String {
        format!("{} {} {:?}", self.token_type, self.lexeme, self.literal_value)
    }
}

impl Display for Token{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ token_type: {}, literal: {:?}, line: {}, position: {} }}", self.token_type, self.literal_value,
        self.line_number, self.position)
    }
}