use crate::lexer::InputReader;
use crate::lexer::Token;
use crate::lexer::{TokenMatcher};
use crate::lexer::matchers::token_matcher_helper::TokenMatcherHelper;
use crate::lexer::str_classifier::str_classifier;
use crate::lexer::TokenType;

pub struct TokenMatcherIf {}

impl<'a> TokenMatcher<'a> for TokenMatcherIf {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        TokenMatcherHelper::match_bound_condition(
            reader,
            2,
            "if",
            TokenType::If,
            |next_char| {
                !str_classifier::is_identifier_char(next_char)
            }
        )
    }
}