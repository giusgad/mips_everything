use std::ops::Range;

use self::register::Register;

pub(crate) mod instruction;
pub(crate) mod register;

pub(crate) trait ValidBitRepr {}
impl ValidBitRepr for Bits<32> {}
impl ValidBitRepr for Bits<6> {}
impl ValidBitRepr for Bits<5> {}

/// Represents up to 32 bits of information
pub(crate) struct Bits<const N: usize>
where
    Self: ValidBitRepr,
{
    data: u32,
}
impl<const N: usize> Bits<N>
where
    Self: ValidBitRepr,
{
    fn new(data: u32) -> Self {
        assert!(data.trailing_zeros() >= 32 - N as u32);
        Self { data }
    }
    fn get(&self) -> u32 {
        self.data
    }
}

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
