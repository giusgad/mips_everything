use std::ops::Range;

use super::{directive::Directive, instruction::InstructionKind, register::Register};

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
    Whitespace,

    // parenthesis
    LParen, // (
    RParen, // )

    // punctuation and operators
    SingleQuote, // \'
    Plus,        // +
    Minus,       // -
    Comma,       // ,
    Dot,         // .
    Colon,       // :

    Register(Register),
    Instruction(InstructionKind),
    Directive(Directive),
    Ident(String),
    String(String),
    Number(i16),
}
