use crate::errors::LexerErrorKind;

use self::defs::{register::Register, Token};
pub mod defs;

#[derive(Debug)]
pub struct Lexer {
    pos: usize,
    in_string: bool,
    input: Vec<u8>,
}

impl Lexer {
    fn new(input: String) -> Self {
        Lexer {
            pos: 0,
            input: input.into_bytes(),
            in_string: false,
        }
    }

    fn next_token(&mut self) -> Result<Token, LexerErrorKind> {
        if self.in_string {
            return Ok(Token::String(self.read_string()?));
        }
        self.skip_whitespace();
        let Some(curr) = self.peek() else {
            return Ok(Token::Eof);
        };
        if let Ok(tok) = curr.try_into() {
            match tok {
                Token::DoubleQuote => {
                    self.in_string = true;
                }
                Token::Backslash => {
                    // WARN: unsure if this works
                    let Some(next) = self.read_next() else {
                        todo!() //error
                    };
                    return next.try_into();
                }
                t => {
                    self.read_next();
                    return Ok(t);
                }
            };
        }
        match curr {
            b'a'..=b'z' | b'A'..=b'Z' => self.read_ident(),
            b'0'..=b'9' | b'-' => self.read_number(),
            b'$' => self.read_register(),
            _ => curr.try_into(),
        }
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
    /// reads the next byte and increment `self.pos`
    fn read_next(&mut self) -> Option<u8> {
        if self.pos >= self.input.len() {
            None
        } else {
            self.pos += 1;
            Some(self.input[self.pos - 1])
        }
    }

    fn read_string(&mut self) -> Result<String, LexerErrorKind> {
        todo!();
    }
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
        if let Ok(num) = string.parse::<i64>() {
            Ok(Token::Number(num))
        } else {
            //TODO: correct error
            Err(LexerErrorKind::InvalidToken('c'))
        }
    }
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
            dbg!(res);
        }
        assert_eq!(lexer.next_token().unwrap(), Token::Eof);
    }
    #[test]
    fn read_strings() {
        //TODO: test
    }
}
