use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // literals
    Identifier,
    String,
    Number,

    // Keywords
    If,             // (OK) if
    Else,           // (OK) else
    Loop,           // (OK) while
    Struct,         // (OK) struct
    Func,           // fn
    Nil,            // nil
    Return,         // return
    This,           // self,
    True,           // true,
    False,          // false
    Var,            // (OK) var
    Continue,       // (OK) continue
    Break,          // (OK) break
    Public,         // public
    ScopeReturn,    // <-
    // other
    Plus,           // (OK) +
    Minus,          // (OK) -
    Star,           // *
    Slash,          // /
    OpenParen,      // (OK) (
    CloseParen,     // (OK) )
    OpenBrace,      // (OK) {
    CloseBrace,     // (OK) }
    SemiColon,      // ;
    Comma,          //,
    Dot,            // .
    AmpAmp,         // (OK) &&
    PipePipe,       // (OK) ||

    Bang,           // !
    BangEqual,      // !=
    EqualEqual,     // (OK) ==
    Equal,          // (OK) =
    Greater,        // >
    GreaterEqual,   // >=
    Less,           // <
    LessEqual,      // <=
    SlashSlash,     // // this needs to ignore rest of line
    SlashStar,      // /*
    StarSlash,      // */
    EOF,
    Illegal,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}