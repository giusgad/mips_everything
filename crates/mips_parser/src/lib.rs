#![allow(dead_code)]

use errors::CompileError;
use lexer::Lexer;

pub mod errors;
pub mod lexer;
mod parser;

pub struct MipsCompiler {
    input: String,
}

type Ast = ();
impl MipsCompiler {
    pub fn new(input: String) -> Self {
        Self { input }
    }
    pub fn compile(self) -> Result<Ast, CompileError> {
        //TODO: bring every part together
        let _tokens = Lexer::new(self.input).lex()?;
        Ok(())
    }
}
