use crate::lexer::defs::register::RegisterParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Syntax error {0}")]
    Lexer(#[from] LexerError),
    // #[error("Parsing error: {0}")]
    // Parser(#[from] ParserError)
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("in line {line}:\n{kind}")]
pub struct LexerError {
    pub kind: LexerErrorKind,
    pub line: usize,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexerErrorKind {
    #[error("Invalid register: \"{0}\"")]
    Register(#[from] RegisterParseError),
    #[error("Invalid token, couldn't read: \"{0}\"")]
    InvalidToken(char),
    #[error("Expected string closing delimiter")]
    ExpectedStringEnd,
    #[error("Number literal \"{0}\" is invalid.")]
    NumberParseError(String),
    #[error("Number out of range \"{0}\", must be between -32768 and 32767")]
    NumberOutOfRange(String),
}
