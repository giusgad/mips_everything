use self::register::Register;

pub(crate) mod register;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Token {
    Eof,
    Newline,

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

    Register(Register),
    Ident(String),
    String(String),
    Number(i16),
}
