use crate::lexer::token::Token;
use crate::lexer::token_matcher::{TokenLocalisation, TokenMatcher};
use crate::lexer::token_type::TokenType;

pub struct CloseParenTokenMatcher{
}

impl<'a> TokenMatcher<'a> for CloseParenTokenMatcher {
    fn is_match(&self, chars: &Vec<&'a char>) -> bool {
        if chars.len() > 1 {
            return false;
        }

        let cond = *chars[0] == ')';
        cond
    }

    fn create_token(&self, chars: &Vec<&'a char>, localisation: TokenLocalisation) -> Token {
        let lexeme = chars.iter().cloned().collect();

        return Token {
            token_type: TokenType::CloseParen,
            line_number: localisation.line,
            position: localisation.line_start,
            lexeme,
            literal_value: None,
        };
    }
}