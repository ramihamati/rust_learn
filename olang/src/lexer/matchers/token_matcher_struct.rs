use crate::lexer::InputReader;
use crate::lexer::Token;
use crate::lexer::{TokenMatcher};
use crate::lexer::matchers::token_matcher_helper::TokenMatcherHelper;
use crate::lexer::str_classifier::str_classifier;
use crate::lexer::TokenType;

pub struct TokenMatcherStruct {}

impl<'a> TokenMatcher<'a> for TokenMatcherStruct {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        TokenMatcherHelper::match_bound_condition(
            reader,
            6,
            "struct",
            TokenType::Struct,
            |next_char| {
                !str_classifier::is_identifier_char(next_char)
            }
        )
    }


}