use crate::errors::LexerErrorKind;

use self::register::Register;

mod register;

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
    Colon,       // :
    Backslash,   // \\

    Register(Register),
}

impl TryFrom<char> for Token {
    type Error = LexerErrorKind;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tok = match value {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '"' => Token::DoubleQuote,
            '\'' => Token::SingleQuote,
            '#' => Token::Sharp,
            '+' => Token::Plus,
            '-' => Token::Minus,
            ',' => Token::Comma,
            ':' => Token::Colon,
            '\\' => Token::Backslash,
            c => return Err(LexerErrorKind::InvalidToken(c)),
        };
        Ok(tok)
    }
}
