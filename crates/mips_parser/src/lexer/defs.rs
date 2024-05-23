use self::register::Register;

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
    Number(i16),
}
