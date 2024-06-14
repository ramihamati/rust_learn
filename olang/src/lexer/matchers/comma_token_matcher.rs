use crate::lexer::readers::code_reader::InputReader;
use crate::lexer::symbols::token::Token;
use crate::lexer::token_matcher::{TokenMatcher};
use crate::lexer::symbols::token_type::TokenType;

pub struct CommaTokenMatcher {
}

impl<'a> TokenMatcher<'a> for CommaTokenMatcher {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        reader.advance(1);
        let peek = reader.collect();

        if let Some(first_char) = peek.chars().next() {

            if (first_char == ','){
                let token = Token {
                    token_type: TokenType::Comma,
                    line_number: reader.line,
                    position: reader.line_start,
                    lexeme: peek.to_string(),
                    literal_value: None,
                };
                reader.forward();

                return Some(token)
            }
        }
        reader.revert_advance();
        return None;
    }
}