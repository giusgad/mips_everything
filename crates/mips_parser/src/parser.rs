use std::{iter::Peekable, slice::Iter};

use crate::defs::{instruction::InstructionKind, token::Token};


pub(crate) struct Parser<'a> {
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    fn do_something(&mut self) {
        let a = self.tokens.next();
        dbg!(self.tokens.next());
    }
}
