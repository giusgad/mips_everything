use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Syntax error {0}")]
    Lexer(#[from] LexerError),
}

#[derive(Debug, Error)]
#[error("at line {line}\n{kind}")]
pub struct LexerError {
    kind: LexerErrorKind,
    line: usize,
}

#[derive(Debug, Error)]
pub enum LexerErrorKind {
    #[error("Invalid register: \"{0}\"")]
    RegisterParseError(String),
}
