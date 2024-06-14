use std::fmt::{Display, Formatter};
use crate::lexer::symbols::literal_value::LiteralValue;
use crate::lexer::symbols::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal_value: Option<LiteralValue>,
    pub position: usize,
    pub line_number: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal_value: Option<LiteralValue>,
        position: usize,
        line_number: usize) -> Self {
        Token {
            token_type,
            literal_value,
            lexeme,
            position,
            line_number
        }
    }

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