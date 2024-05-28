use std::ops::Range;

use crate::lexer::defs::register::RegisterParseError;
use ariadne::{sources, Config, IndexType, Label, Report, ReportKind};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Syntax error {0}")]
    Lexer(#[from] LexerError),
    // #[error("Parsing error: {0}")]
    // Parser(#[from] Parser Error)
}

impl CompileError {
    pub fn display_formatted(
        &self,
        file_name: &'static str,
        file_content: &str,
    ) -> std::io::Result<()> {
        Report::build(ReportKind::Error, file_name, 0)
            .with_config(Config::default().with_index_type(IndexType::Byte))
            .with_message(self.variant_str())
            .with_label(
                Label::new((file_name, self.get_target_range())).with_message(self.get_message()),
            )
            .finish()
            .eprint(sources(vec![(file_name, file_content)]))?;
        Ok(())
    }

    fn get_target_range(&self) -> Range<usize> {
        match self {
            CompileError::Lexer(err) => err.target_range.clone(),
        }
    }

    fn variant_str(&self) -> &str {
        match self {
            CompileError::Lexer(_) => "Syntax error",
        }
    }

    fn get_message(&self) -> String {
        match self {
            CompileError::Lexer(err) => err.kind.to_string(),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("{kind}")]
pub struct LexerError {
    pub kind: LexerErrorKind,
    // The range of the bytes that caused the error
    pub target_range: Range<usize>,
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
