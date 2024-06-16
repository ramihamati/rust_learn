use crate::lexer::{InputReader, LiteralValue};
use crate::lexer::Token;
use crate::lexer::{TokenMatcher};
use crate::lexer::matchers::token_matcher_helper::TokenMatcherHelper;
use crate::lexer::TokenType;

pub struct TokenMatcherCommentLine {}

impl<'a> TokenMatcher<'a> for TokenMatcherCommentLine {
    fn create(&self, reader: &mut InputReader) -> Option<Token> {
        TokenMatcherHelper::match_symbol_extract_literal(
            reader,
            "//",
            TokenType::SlashSlash,
            |reader|{
                return if (reader.advance_until_end_of_line()) {
                    let rest_of_line = reader.collect().to_string();
                    let literalValue = LiteralValue::StringValue(String::from(rest_of_line));
                    Some(literalValue)
                } else {
                    None
                }

            }
        )

    }
}