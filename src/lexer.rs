#![allow(dead_code)]
use self::defs::{register::Register, Token};
use crate::errors::{LexerError, LexerErrorKind};

pub mod defs;

#[derive(Debug)]
pub struct Lexer {
    pos: usize,
    line: usize,
    /// encountered a ", next token is going to be a string
    in_string: bool,
    /// last token was a string
    returned_string: bool,
    input: Vec<u8>,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            pos: 0,
            line: 0,
            input: input.into_bytes(),
            in_string: false,
            returned_string: false,
        }
    }

    /// returns a Vec of all the tokens from the input
    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut res = Vec::new();
        loop {
            match self.next_token() {
                Ok(tok) => {
                    if tok == Token::Eof {
                        res.push(tok);
                        return Ok(res);
                    }
                    res.push(tok)
                }
                Err(err) => {
                    let line = self
                        .input
                        .iter()
                        .take(self.pos)
                        .filter(|&c| *c == 0xA)
                        .count();
                    return Err(LexerError { kind: err, line });
                }
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, LexerErrorKind> {
        if self.in_string && !self.returned_string {
            self.returned_string = true;
            return Ok(Token::String(self.read_string()?));
        }
        self.skip_whitespace();
        let Some(curr) = self.peek() else {
            return Ok(Token::Eof);
        };
        let res = match curr {
            b'(' => Ok(Token::LParen),
            b')' => Ok(Token::RParen),
            b'"' => {
                self.returned_string = false;
                self.in_string = !self.in_string;
                Ok(Token::DoubleQuote)
            }
            b'\'' => Ok(Token::SingleQuote),
            b'#' => Ok(Token::Sharp),
            b'+' => Ok(Token::Plus),
            b'-' => Ok(Token::Minus),
            b',' => Ok(Token::Comma),
            b':' => Ok(Token::Colon),
            b'.' => Ok(Token::Dot),
            b'\\' => Ok(Token::Backslash), // TODO: escaping
            b'a'..=b'z' | b'A'..=b'Z' => return self.read_ident(),
            b'0'..=b'9' => return self.read_number(),
            b'$' => return self.read_register(),
            c => Err(LexerErrorKind::InvalidToken(c.into())),
        };
        self.read_next();
        res
    }

    /// Increments the position until the next character to be read is not whitespace
    fn skip_whitespace(&mut self) {
        while self.peek().is_some_and(|c| c.is_ascii_whitespace()) {
            self.read_next();
        }
    }

    /// Returns the byte that would be read next without altering the state of the Lexer.
    fn peek(&self) -> Option<u8> {
        if self.pos >= self.input.len() {
            None
        } else {
            Some(self.input[self.pos])
        }
    }
    /// Reads the next byte and increment `self.pos`
    fn read_next(&mut self) -> Option<u8> {
        if self.pos >= self.input.len() {
            None
        } else {
            self.pos += 1;
            Some(self.input[self.pos - 1])
        }
    }

    /// Reads a string, stops when a non escaped closing quote is found, returns an error if the
    /// closing delimiter doesn't exist
    fn read_string(&mut self) -> Result<String, LexerErrorKind> {
        let mut string = String::new();
        let mut escaped = false;
        while let Some(c) = self.peek() {
            if escaped {
                escaped = false;
                //TODO: special chars like \n
                string.push(c as char);
                self.read_next();
                continue;
            } else if c == b'"' {
                break;
            } else if c == b'\\' {
                escaped = true;
                self.read_next();
                continue;
            }
            self.read_next();
            string.push(c as char)
        }
        // Self.peek() should be '"' because the next token will be a closing quote,
        // if it's not the string is not closed and it's an error
        if self.peek() != Some(b'"') {
            return Err(LexerErrorKind::ExpectedStringEnd);
        }
        Ok(string)
    }

    /// Reads an ident, keeps going until an ascii whitespace character is found
    fn read_ident(&mut self) -> Result<Token, LexerErrorKind> {
        let mut string = String::new();
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            self.read_next();
            string.push(c as char);
        }
        Ok(Token::Ident(string))
    }

    fn read_number(&mut self) -> Result<Token, LexerErrorKind> {
        let mut string = String::new();
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            self.read_next();
            string.push(c as char);
        }
        // TODO: parse also bin and hex
        if let Ok(num) = string.parse::<u64>() {
            Ok(Token::Number(num))
        } else {
            //TODO: correct error
            Err(LexerErrorKind::InvalidToken('c'))
        }
    }

    /// Reads a register starting from a dollar sign, returning a [`Token::Register`], containing the representation of the
    /// register following the $
    fn read_register(&mut self) -> Result<Token, LexerErrorKind> {
        let mut chars = Vec::new();
        // skip the $ itself
        self.read_next();
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            self.read_next();
            chars.push(c as char);
        }
        Ok(Token::Register(Register::try_from(chars.as_slice())?))
    }
}

#[cfg(test)]
mod test {
    use test::defs::register::{RegisterName, RegisterParseError, RegisterParseErrorKind};

    use self::defs::register::RegisterPrefixedName;

    use super::*;
    #[test]
    fn read_tokens() {
        let input = ".data
x: .word 7
y: .word 3
.text
la $s0 x
la $a0 mylabel
li $v0 4
syscall			
";
        let tokens = [
            Token::Dot,
            Token::Ident("data".into()),
            Token::Ident("x".into()),
            Token::Colon,
            Token::Dot,
            Token::Ident("word".into()),
            Token::Number(7),
            Token::Ident("y".into()),
            Token::Colon,
            Token::Dot,
            Token::Ident("word".into()),
            Token::Number(3),
            Token::Dot,
            Token::Ident("text".into()),
            Token::Ident("la".into()),
            Token::Register(Register::PrefixedNumber(
                RegisterPrefixedName::new_unchecked('s', 0),
            )),
            Token::Ident("x".into()),
            Token::Ident("la".into()),
            Token::Register(Register::PrefixedNumber(
                RegisterPrefixedName::new_unchecked('a', 0),
            )),
            Token::Ident("mylabel".into()),
            Token::Ident("li".into()),
            Token::Register(Register::PrefixedNumber(
                RegisterPrefixedName::new_unchecked('v', 0),
            )),
            Token::Number(4),
            Token::Ident("syscall".into()),
            Token::Eof,
        ];
        let mut lexer = Lexer::new(input.into());
        for res in tokens.into_iter() {
            assert_eq!(lexer.next_token().unwrap(), res);
        }
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }
    #[test]
    fn read_strings() {
        let input = r#"
        data "inside string"
        out "inside \" escaped"
        double "inside\"some\"double"
        "#;
        let tokens = [
            Token::Ident("data".into()),
            Token::DoubleQuote,
            Token::String("inside string".into()),
            Token::DoubleQuote,
            Token::Ident("out".into()),
            Token::DoubleQuote,
            Token::String("inside \" escaped".into()),
            Token::DoubleQuote,
            Token::Ident("double".into()),
            Token::DoubleQuote,
            Token::String("inside\"some\"double".into()),
            Token::DoubleQuote,
        ];
        let mut lexer = Lexer::new(input.into());
        for res in tokens.into_iter() {
            assert_eq!(lexer.next_token().unwrap(), res);
        }
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
        let mut lexer = Lexer::new("\"Open string".into());
        assert_eq!(lexer.next_token(), Ok(Token::DoubleQuote));
        assert_eq!(lexer.next_token(), Err(LexerErrorKind::ExpectedStringEnd));
        assert_eq!(lexer.next_token(), Ok(Token::Eof));
    }
    #[test]
    fn lex() {
        let input = "lw $ra 4";
        let mut lexer = Lexer::new(input.into());
        let tokens = vec![
            Token::Ident("lw".into()),
            Token::Register(Register::Name(RegisterName::Ra)),
            Token::Number(4),
            Token::Eof,
        ];
        assert_eq!(lexer.lex(), Ok(tokens));
        let input = "lw 4
iden
lw $error
test";
        let mut lexer = Lexer::new(input.into());
        assert_eq!(
            lexer.lex(),
            Err(LexerError {
                kind: LexerErrorKind::Register(RegisterParseError {
                    kind: RegisterParseErrorKind::InvalidPrefix('c'),
                    reg: "".into(),
                }),
                line: 3
            })
        )
    }
}
