use self::register::Register;
use crate::errors::LexerErrorKind;

pub mod register;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Eof,

    // parenthesis
    LParen, // (
    RParen, // )

    // punctuation and operators
    DoubleQuote, // "
    SingleQuote, // \'
    Sharp,       // #
    Plus,        // +
    Minus,       // -
    Comma,       // ,
    Dot,         // .
    Colon,       // :
    Backslash,   // \\

    Register(Register),
    Ident(String),
    String(String),
    Number(i64),
}

impl TryFrom<u8> for Token {
    type Error = LexerErrorKind;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let tok = match value {
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'"' => Token::DoubleQuote,
            b'\'' => Token::SingleQuote,
            b'#' => Token::Sharp,
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b',' => Token::Comma,
            b':' => Token::Colon,
            b'.' => Token::Dot,
            b'\\' => Token::Backslash,
            c => return Err(LexerErrorKind::InvalidToken(c.into())),
        };
        Ok(tok)
    }
}
