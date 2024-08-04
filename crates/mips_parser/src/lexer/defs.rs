use std::ops::Range;

use self::register::Register;

pub(crate) mod register;
pub(crate) mod instruction;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: Range<usize>,
}

impl Token {
    pub(crate) fn new(kind: TokenKind, span: Range<usize>) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum TokenKind {
    Eof,
    Newline,

    // parenthesis
    LParen, // (
    RParen, // )

    // punctuation and operators
    SingleQuote, // \'
    Sharp,       // #
    Plus,        // +
    Minus,       // -
    Comma,       // ,
    Dot,         // .
    Colon,       // :

    Register(Register),
    Ident(String),
    String(String),
    Number(i16),
}
