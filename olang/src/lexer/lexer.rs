use crate::lexer::code_reader::InputReader;
use crate::lexer::literal_value::LiteralValue;
use crate::lexer::matchers::close_paren_token_matcher::CloseParenTokenMatcher;
use crate::lexer::matchers::open_paren_token_matcher::OpenParenTokenMatcher;
use crate::lexer::token::Token;
use crate::lexer::token_matcher::{TokenLocalisation, TokenMatcher};
use crate::lexer::token_type::TokenType;

pub struct Lexer<'a> {
    tokens: Vec<Token>,
    reader: InputReader<'a>,
}


impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Lexer {
            tokens: vec![],
            reader: InputReader::new(input),
        }
    }

    pub fn scan_tokens(self: &mut Self) -> Result<Vec<Token>, String> {
        let mut errors = vec![];

        while self.reader.scanner_forward() { // forward scanner start/end to end
            if self.reader.scanner_advance() {
                let current_char = self.reader.get_current_char();

                match self.scan_token(&current_char) {
                    Ok(token) => {
                        self.tokens.push(token)
                    },
                    Err(err) => {
                        errors.push(err);
                    }
                };
            }
        }

        self.add_eof();
        if errors.len() > 0 {
            let mut joined = "".to_string();
            for err in errors {
                joined.push_str(&err);
                joined.push_str("\n");
            }

            return Err(joined);
        }
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

    fn scan_token(self: &Self, c : &char) -> Result<Token, String> {
        let mut matchers : Vec<Box<dyn TokenMatcher>> = vec![];

        matchers.push(Box::new(OpenParenTokenMatcher {}));
        matchers.push(Box::new(CloseParenTokenMatcher {}));

        for matcher in matchers {
            let vec_chars = Self::to_vec(c);

            if (matcher.is_match(&vec_chars)){
                let token = matcher.create_token(&vec_chars, TokenLocalisation{
                    line_start: self.reader.line_start,
                    line: self.reader.line,
                    line_end: self.reader.line_current
                });

                return Ok(token)
            }
        }
        return match c {
            '{' => Ok(self.create_token(TokenType::OpenBrace, None)),
            '}' => Ok(self.create_token(TokenType::CloseBrace, None)),
            ',' => Ok(self.create_token(TokenType::Comma, None)),
            '.' => Ok(self.create_token(TokenType::Dot, None)),
            '+' => Ok(self.create_token(TokenType::Plus, None)),
            '-' => Ok(self.create_token(TokenType::Minus, None)),
            ';' => Ok(self.create_token(TokenType::SemiColon, None)),
            '*' => Ok(self.create_token(TokenType::Star, None)),
            _ => {
                return Err(format!("unrecognized char at line {} position {}: {}", self.reader.line, self.reader.scanner_current, c));
            }
        };
    }

    fn to_vec(c : &char) -> Vec<&char>{
        // temporary hack
        let mut arr : Vec<&char> = vec![];
        arr.push(c);
        arr
    }
    fn create_token(self: &Self, token_type: TokenType, literal: Option<LiteralValue>) -> Token {
        return Token {
            token_type,
            line_number: self.reader.line,
            position: self.reader.line_current,
            lexeme: self.reader.get_lexeme(),
            literal_value: literal,
        };
    }
}