use std::ops::Range;

use crate::defs::register::RegisterParseError;
use ariadne::{sources, Config, IndexType, Label, Report, ReportKind};
use thiserror::Error;

/// This trait implements functions that define how an error is displayed with [`ariadne`].
pub(crate) trait AriadneError {
    /// The general message that goes before the code snippet.
    /// Acts like the "title" of the error.
    fn general_message(&self) -> String;
    /// The text that goes on the label connected to the code snippet
    fn label(&self) -> String;
    /// A note to add after the code snippet
    fn note(&self) -> Option<String> {
        None
    }
}

#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Syntax error {0}")]
    Lexer(#[from] LexerError),
    // #[error("Parsing error: {0}")]
    // Parser(#[from] Parser Error)
}

impl CompileError {
    pub fn display_formatted(&self, file_name: String, file_content: &str) -> std::io::Result<()> {
        let mut report = Report::build(
            ReportKind::Error,
            file_name.clone(),
            self.get_span()
                .next()
                .expect("Error span should be pointing to at least one byte."),
        )
        .with_config(Config::default().with_index_type(IndexType::Byte))
        .with_message(self.general_message());
        report.add_label(
            Label::new((file_name.clone(), self.get_span())).with_message(self.label_message()),
        );
        if let Some(note) = self.get_note() {
            report.set_note(note);
        }
        report
            .finish()
            .eprint(sources(vec![(file_name, file_content)]))?;
        Ok(())
    }

    fn get_note(&self) -> Option<String> {
        match self {
            CompileError::Lexer(err) => err.kind.note(),
        }
    }

    fn general_message(&self) -> String {
        match self {
            CompileError::Lexer(err) => err.kind.general_message(),
        }
    }

    fn get_span(&self) -> Range<usize> {
        match self {
            CompileError::Lexer(err) => err.span.clone(),
        }
    }

    fn label_message(&self) -> String {
        match self {
            CompileError::Lexer(err) => err.kind.label(),
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("{kind}")]
pub struct LexerError {
    pub kind: LexerErrorKind,
    // The span of bytes that caused the error
    pub span: Range<usize>,
}

impl LexerError {
    pub fn new(kind: LexerErrorKind, span: Range<usize>) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum LexerErrorKind {
    #[error("Invalid register: {0}.")]
    Register(#[from] RegisterParseError),
    #[error("Invalid token: \"{0}\".")]
    InvalidToken(char),
    #[error("Non ascii character.")]
    NonAsciiChar,
    #[error("Expected string closing delimiter.")]
    ExpectedStringEnd,
    #[error("Number literal is invalid.")]
    NumberParseError,
    #[error("Number out of range.")]
    NumberOutOfRange,
}

impl AriadneError for LexerErrorKind {
    fn general_message(&self) -> String {
        format!("{self}")
    }
    fn label(&self) -> String {
        match self {
            LexerErrorKind::Register(err) => err.label(),
            LexerErrorKind::InvalidToken(_) => "This token is invalid".into(),
            LexerErrorKind::NonAsciiChar => "This token is invalid as its not ascii".into(),
            LexerErrorKind::ExpectedStringEnd => "The string should be closed".into(),
            LexerErrorKind::NumberParseError => "This number/address is not valid".into(),
            LexerErrorKind::NumberOutOfRange => "This number is out of range".into(),
        }
    }
    fn note(&self) -> Option<String> {
        match self {
            LexerErrorKind::Register(err) => err.note(),
            LexerErrorKind::ExpectedStringEnd => {
                Some("The quote that should close the string is missing.".into())
            }
            LexerErrorKind::NumberOutOfRange => Some("The number is represented with 16 bits, therefore it must be between -32768 and 32767".into()),
            _ => None,
        }
    }
}
