use crate::lexer::defs::Token;

pub struct Parser {
    input: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { input: tokens }
    }
}
