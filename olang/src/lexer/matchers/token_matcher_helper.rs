use crate::lexer::{InputReader, Token, TokenType};

pub struct TokenMatcherHelper {}

impl TokenMatcherHelper {
    pub fn match_single_character(reader: &mut InputReader, character: char, token_type: TokenType) -> Option<Token> {
        if !reader.advance(1) {
            return None;
        }
        let peek = reader.collect();

        return if let Some(first_char) = peek.chars().next() {
            if first_char == character {
                let token = Token {
                    token_type,
                    line_number: reader.line,
                    position: reader.line_start,
                    lexeme: peek.to_string(),
                    literal_value: None,
                };
                reader.forward();
                Some(token)
            } else {
                reader.revert_advance();
                None
            }
        } else {
            reader.revert_advance();
            None
        };
    }

    pub fn match_no_bound_condition(reader: &mut InputReader, count: usize, matcher: &str, token_type: TokenType) -> Option<Token> {
        if (!reader.advance(count)) {
            return None;
        }

        let peek = reader.collect();

        if peek == matcher {
            let token = Token {
                token_type,
                line_number: reader.line,
                position: reader.line_start,
                lexeme: peek.to_string(),
                literal_value: None,
            };
            reader.forward();

            return Some(token);
        }

        reader.revert_advance();
        return None;
    }

    pub fn match_bound_condition<F>(reader: &mut InputReader, count: usize, matcher: &str, token_type: TokenType, valid_next_char: F) -> Option<Token>
        where F: Fn(char) -> bool
    {
        if !reader.advance(count) {
            return None;
        }

        // collect before peeking next char
        let collected = reader.collect();

        if (collected != matcher) {
            reader.revert_advance();
            return None;
        }

        let is_valid_next_char = match reader.peek_one() {
            Some(nextChar) => {
                if valid_next_char(nextChar){
                    // it would be an iff or if_ or if1
                    true
                } else {
                    false
                }
            }
            None => {
                true
            }
        };

        if collected == matcher && is_valid_next_char {
            let token = Token {
                token_type: token_type,
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