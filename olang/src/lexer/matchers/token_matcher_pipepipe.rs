use crate::lexer::InputReader;
use crate::lexer::Token;
use crate::lexer::{TokenMatcher};
use crate::lexer::matchers::token_matcher_helper::TokenMatcherHelper;
use crate::lexer::TokenType;

pub struct TokenMatcherPipePipe {
}

impl<'a> TokenMatcher<'a> for TokenMatcherPipePipe {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        TokenMatcherHelper::match_no_bound_condition(
            reader,
            2,
            "||",
            TokenType::PipePipe
        )
    }
}