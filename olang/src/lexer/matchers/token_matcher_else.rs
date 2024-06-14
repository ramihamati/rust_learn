use crate::lexer::InputReader;
use crate::lexer::Token;
use crate::lexer::{TokenMatcher};
use crate::lexer::TokenType;

pub struct TokenMatcherElse {}

impl<'a> TokenMatcher<'a> for TokenMatcherElse {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        if !reader.advance(4) {
            return None;
        }

        // collect before peeking next char
        let collected = reader.collect();

        if (collected != "else"){
            reader.revert_advance();
            return None;
        }

        let is_valid_next_char = match reader.peek_one() {
            Some(nextChar) => {
                if reader.is_identifier_char(nextChar) {
                    // it would be an elsef or else_ or else1
                    false
                }else{
                    true
                }
            },
            None => {
                true
            }
        };

        if collected == "else" && is_valid_next_char{
            let token = Token {
                token_type: TokenType::Else,
                line_number: reader.line,
                position: reader.line_start,
                lexeme: collected.to_string(),
                literal_value: None,
            };
            reader.forward();
            return Some(token);
        }

        reader.revert_advance();
        return None;
    }


}