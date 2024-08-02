use crate::lexer::defs::Token;

pub(crate) struct Parser {
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens }
    }
}
