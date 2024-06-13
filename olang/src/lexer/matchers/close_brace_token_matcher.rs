use crate::lexer::readers::code_reader::InputReader;
use crate::lexer::token::Token;
use crate::lexer::token_matcher::{TokenMatcher};
use crate::lexer::token_type::TokenType;

pub struct CloseBraceTokenMatcher {
}

impl<'a> TokenMatcher<'a> for CloseBraceTokenMatcher {

    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        if (!reader.advance(1)){
            return None
        }
        let peek = reader.collect();

        if let Some(first_char) = peek.chars().next() {
            if (first_char == '}'){
                let token = Token {
                    token_type: TokenType::CloseBrace,
                    line_number: reader.line,
                    position: reader.line_start,
                    lexeme: peek.to_string(),
                    literal_value: None,
                };
                reader.forward();
                return Some(token)
            }
            else{
                reader.revert_advance();
                return None;
            }
        }
        else{
            reader.revert_advance();
            return None;
        }

    }
}