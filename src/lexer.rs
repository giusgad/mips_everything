pub mod defs;

pub struct Lexer {
    pos: usize,
    input: Vec<u8>,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            pos: 0,
            input: input.into_bytes(),
        }
    }
}
