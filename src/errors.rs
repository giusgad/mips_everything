use crate::lexer::defs::register::RegisterParseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Syntax error {0}")]
    Lexer(#[from] LexerError),
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("at line {line}\n{kind}")]
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
}
