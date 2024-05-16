#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Eof,

    // parenthesis
    LParen, // (
    RParen, // )

    // punctuation and operators
    DoubleQuote, // "
    SingleQuote, // \'
    Sharp,       // #
    Plus,        // +
    Minus,       // -
    Comma,       // ,
    Colon,       // :
    Backslash,   // \\

    Register(Register),
}

impl TryFrom<char> for Token {
    //TODO: use my error (or maybe not this should be internal)
    type Error = std::io::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tok = match value {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '"' => Token::DoubleQuote,
            '\'' => Token::SingleQuote,
            '#' => Token::Sharp,
            '+' => Token::Plus,
            '-' => Token::Minus,
            ',' => Token::Comma,
            ':' => Token::Colon,
            '\\' => Token::Backslash,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Invalid token",
                ))
            }
        };
        Ok(tok)
    }
}

#[derive(Debug, PartialEq, Eq)]
/// Identifies a valid register in the CPU
enum Register {
    Number(u8),
    Name(RegisterName),
}

#[derive(Debug, PartialEq, Eq)]
struct RegisterName {
    prefix: char,
    index: u8,
}

impl TryFrom<&str> for RegisterName {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}
