use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    // literals
    Identifier,
    String,
    Number,

    // Keywords
    If, // if
    Else, // else
    Loop, // while
    And, // and
    Struct,
    Func, // fn
    Nil, // nil
    Or, // or
    Return, // return
    This, // self,
    True, // true,
    False, // false
    Var, // var
    // other
    Plus, // + (OK)
    Minus, // - (OK)
    Star, // *
    Slash, // /
    OpenParen, // (
    CloseParen, // )
    OpenBrace, // {
    CloseBrace, // }
    SemiColon, // ;
    Assign, // =
    Comma, //,
    Dot, // .
    LogicalAnd, // &&

    Bang, // !
    BangEqual, // !=
    EqualEqual, // ==
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