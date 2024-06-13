use crate::lexer::readers::code_reader::InputReader;
use crate::lexer::matchers::close_brace_token_matcher::CloseBraceTokenMatcher;
use crate::lexer::matchers::close_paren_token_matcher::CloseParenTokenMatcher;
use crate::lexer::matchers::comma_token_matcher::CommaTokenMatcher;
use crate::lexer::matchers::equalequal_token_matcher::EqualEqualTokenMatcher;
use crate::lexer::matchers::open_brace_token_matcher::OpenBraceTokenMatcher;
use crate::lexer::matchers::open_paren_token_matcher::OpenParenTokenMatcher;
use crate::lexer::token::Token;
use crate::lexer::token_matcher::{TokenMatcher};
use crate::lexer::token_type::TokenType;

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    reader: InputReader<'a>,
    matchers : Vec<Box<dyn TokenMatcher<'a>>>
}


impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        let mut matchers : Vec<Box<dyn TokenMatcher>> = vec![];

        // order as priority matters
        matchers.push(Box::new(OpenParenTokenMatcher {}));
        matchers.push(Box::new(CloseParenTokenMatcher {}));
        matchers.push(Box::new(OpenBraceTokenMatcher {}));
        matchers.push(Box::new(CloseBraceTokenMatcher {}));
        matchers.push(Box::new(CommaTokenMatcher {}));
        matchers.push(Box::new(EqualEqualTokenMatcher {}));

        Lexer {
            tokens: vec![],
            reader: InputReader::new(input),
            matchers
        }
    }

    pub fn get_next_token(self: &mut Self) -> Option<Token>{
        for matcher in &self.matchers {
            let res = matcher.create(&mut self.reader);
            match res{
                None => {
                    continue;
                }
                Some(token) => {
                    return Some(token);
                }
            }
        }

        return None
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        // let mut errors = vec![];
        let mut unidentifierd = String::new();

        while(self.reader.can_advance()) {
            let next_token = self.get_next_token();
            match next_token {
                Some(token) =>{
                    self.tokens.push(token);
                }
                None =>{
                    self.reader.advance(1);
                    unidentifierd = format!("{}{}", unidentifierd, self.reader.collect());
                    self.reader.forward();
                }
            }

            // errors.push("failed to match");
            // self.reader.scanner_advance(1);
            // self.reader.scanner_forward();
        }

        self.add_eof();
        // if errors.len() > 0 {
        //     let mut joined = "".to_string();
        //     for err in errors {
        //         joined.push_str(&err);
        //         joined.push_str("\n");
        //     }
        //
        //     return Err(joined);
        // }
        println!("unidentified {}", unidentifierd);
        Ok(self.tokens.clone())
    }

    fn add_eof(self: &mut Self) {
        self.tokens.push(Token {
            position: self.reader.scanner_current,
            line_number: self.reader.line,
            literal_value: None,
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
        });
    }

}