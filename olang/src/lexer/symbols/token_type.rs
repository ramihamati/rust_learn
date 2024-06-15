use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // literals
    Identifier,
    String,
    Number,

    // Keywords
    If, // if (OK)
    Else, // else (OK)
    Loop, // while
    Struct,
    Func, // fn
    Nil, // nil
    Return, // return
    This, // self,
    True, // true,
    False, // false
    Var, // var or let?
    Continue,
    Break,
    ScopeReturn, // <-
    // other
    Plus, // + (OK)
    Minus, // - (OK)
    Star, // *
    Slash, // /
    OpenParen, // ( (OK)
    CloseParen, // ) (OK)
    OpenBrace, // { (OK)
    CloseBrace, // } (OK)
    SemiColon, // ;
    Comma, //,
    Dot, // .
    AmpAmp, // && (OK)
    PipePipe, // || (OK)

    Bang, // !
    BangEqual, // !=
    EqualEqual, // == (OK)
    Equal, // = (OK)
    Greater, // >
    GreaterEqual, // >=
    Less, // <
    LessEqual, // <=

    EOF,
    Illegal,
}

impl Display for TokenType{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}