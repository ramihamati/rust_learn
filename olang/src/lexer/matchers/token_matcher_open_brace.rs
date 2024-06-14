use crate::lexer::InputReader;
use crate::lexer::Token;
use crate::lexer::{TokenMatcher};
use crate::lexer::TokenType;

pub struct TokenMatcherOpenBrace {
}

impl<'a> TokenMatcher<'a> for TokenMatcherOpenBrace {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        if (!reader.advance(1)){
            return None
        }
        let peek = reader.collect();

        if let Some(first_char) = peek.chars().next() {

            if (first_char == '{'){
                let token = Token {
                    token_type: TokenType::OpenBrace,
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