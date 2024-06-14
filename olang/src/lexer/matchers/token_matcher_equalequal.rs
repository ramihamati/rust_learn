use crate::lexer::InputReader;
use crate::lexer::Token;
use crate::lexer::{TokenMatcher};
use crate::lexer::TokenType;

pub struct TokenMatcherEqualEqual {
}

impl<'a> TokenMatcher<'a> for TokenMatcherEqualEqual {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        reader.advance(2);
        let peek = reader.collect();

        if peek == "==" {
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