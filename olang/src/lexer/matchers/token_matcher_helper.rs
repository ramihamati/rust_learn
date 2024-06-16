use crate::lexer::{InputReader, LiteralValue, Token, TokenType};

pub struct TokenMatcherHelper {}

impl TokenMatcherHelper {

    pub fn match_character(reader: &mut InputReader, character: char, token_type: TokenType) -> Option<Token> {
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

    pub fn match_symbol(reader: &mut InputReader, symbol: &str, token_type: TokenType) -> Option<Token> {
        let count = symbol.len();

        if (!reader.advance(count)) {
            return None;
        }

        let peek = reader.collect();

        if peek == symbol {
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

    pub fn match_symbol_extract_literal<F>(reader: &mut InputReader, symbol: &str, token_type: TokenType, extract : F) -> Option<Token>
    where F : Fn(&mut InputReader) -> Option<LiteralValue> {
        let count = symbol.len();

        if (!reader.advance(count)) {
            return None;
        }

        let peek = reader.collect();

        if peek == symbol {
            let line_number = reader.line;
            let position = reader.line_start;
            let lexeme = peek.to_string();
            reader.forward();

            let literal_value = extract(reader);

            let token = Token {
                token_type,
                line_number,
                position,
                lexeme,
                literal_value,
            };
            reader.forward();
            return Some(token);
        }

        reader.revert_advance();
        return None;
    }

    pub fn match_symbol_bounded_by<F>(reader: &mut InputReader, symbol: &str, token_type: TokenType, valid_next_char: F) -> Option<Token>
        where F: Fn(char) -> bool
    {
        let count = symbol.len();

        if !reader.advance(count) {
            return None;
        }

        // collect before peeking next char
        let collected = reader.collect();

        if (collected != symbol) {
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

        if collected == symbol && is_valid_next_char {
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