#![allow(dead_code)]

use errors::CompileError;
use lexer::Lexer;

mod defs;
mod errors;
mod lexer;
mod parser;

pub struct MipsCompiler<'a> {
    input: &'a str,
}

impl<'a> MipsCompiler<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
    pub fn compile(self) -> Result<(), CompileError> {
        //TODO: bring every part together
        let _tokens = Lexer::new(self.input).lex()?;
        dbg!(_tokens);
        Ok(())
    }
}
