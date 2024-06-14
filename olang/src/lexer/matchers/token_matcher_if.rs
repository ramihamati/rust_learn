use crate::lexer::InputReader;
use crate::lexer::Token;
use crate::lexer::{TokenMatcher};
use crate::lexer::TokenType;

pub struct TokenMatcherIf {}

impl<'a> TokenMatcher<'a> for TokenMatcherIf {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        if !reader.advance(2) {
            return None;
        }

        // collect before peeking next char
        let collected = reader.collect();

        if (collected != "if"){
            reader.revert_advance();
            return None;
        }

        let is_valid_next_char = match reader.peek_one() {
            Some(nextChar) => {
                if reader.is_identifier_char(nextChar) {
                    // it would be an iff or if_ or if1
                    false
                }else{
                    true
                }
            },
            None => {
                true
            }
        };

        if collected == "if" && is_valid_next_char{
            let token = Token {
                token_type: TokenType::If,
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