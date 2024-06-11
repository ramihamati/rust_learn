use crate::lexer::token::Token;
use crate::lexer::token_matcher::{TokenLocalisation, TokenMatcher};
use crate::lexer::token_type::TokenType;

pub struct CommaTokenMatcher {
}

impl<'a> TokenMatcher<'a> for CommaTokenMatcher {
    fn is_match(&self, chars: &'a str) -> bool {
        if chars.len() > 1 {
            return false;
        }

        return if let Some(first_char) = chars.chars().next() {
            first_char == ','
        } else {
            false
        }
    }

    fn create_token(&self, chars: &'a str, localisation: TokenLocalisation) -> Token {
        return Token {
            token_type: TokenType::OpenParen,
            line_number: localisation.line,
            position: localisation.line_start,
            lexeme: chars.to_string(),
            literal_value: None,
        };
    }
}