use crate::lexer::readers::code_reader::InputReader;
use crate::lexer::token::Token;
use crate::lexer::token_matcher::{TokenMatcher};
use crate::lexer::token_type::TokenType;

pub struct EqualEqualTokenMatcher {
}

impl<'a> TokenMatcher<'a> for EqualEqualTokenMatcher {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        reader.advance(2);
        let peek = reader.collect();

        if (peek == "=="){
                let token = Token {
                    token_type: TokenType::EqualEqual,
                    line_number: reader.line,
                    position: reader.line_start,
                    lexeme: peek.to_string(),
                    literal_value: None,
                };
                reader.forward();

                return Some(token)
        }

        reader.revert_advance();
        return None;
    }
}