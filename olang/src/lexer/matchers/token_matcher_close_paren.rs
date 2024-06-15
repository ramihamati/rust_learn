use crate::lexer::InputReader;
use crate::lexer::Token;
use crate::lexer::{TokenMatcher};
use crate::lexer::matchers::token_matcher_helper::TokenMatcherHelper;
use crate::lexer::TokenType;

pub struct TokenMatcherCloseParen {
}

impl<'a> TokenMatcher<'a> for TokenMatcherCloseParen {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        TokenMatcherHelper::match_single_character(reader, ')', TokenType::CloseParen)
    }
}