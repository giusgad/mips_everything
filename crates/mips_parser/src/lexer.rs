use crate::defs::directive::Directive;
use crate::defs::instruction::InstructionKind;
use crate::defs::register::Register;
use crate::defs::token::{Token, TokenKind};
use crate::errors::{LexerError, LexerErrorKind};
use std::num::IntErrorKind;

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    pos: usize,
    input: &'a [u8],
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            pos: 0,
            input: input.as_bytes(),
        }
    }

    /// returns a Vec of all the tokens from the input
    pub fn lex(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut res = Vec::new();
        loop {
            let tok = self.next_token()?;
            if tok.kind == TokenKind::Eof {
                res.push(tok);
                return Ok(res);
            }
            res.push(tok)
        }
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        // skip whitespace and return newline token if needed
        if let Some(tok) = self.skip_whitespace() {
            return Ok(tok);
        }
        let Some(curr) = self.peek() else {
            return Ok(Token::new(TokenKind::Eof, 0..1));
        };
        let kind = match curr {
            b'(' => TokenKind::LParen,
            b')' => TokenKind::RParen,
            b'"' => return self.read_string(),
            b'\'' => TokenKind::SingleQuote,
            b'#' => TokenKind::Sharp,
            b'+' => TokenKind::Plus,
            b'-' => TokenKind::Minus,
            b',' => TokenKind::Comma,
            b':' => TokenKind::Colon,
            b'.' => {
                if let Ok(tok) = self.read_directive() {
                    return Ok(tok);
                } else {
                    TokenKind::Dot
                }
            }
            b'a'..=b'z' | b'A'..=b'Z' => return self.read_ident_or_instruction(),
            b'0'..=b'9' => return self.read_number(),
            b'$' => return self.read_register(),
            c if !c.is_ascii() => {
                // get the lenght of the non ascii character by checking the leading byte
                let length = if c & 0b1110_0000 == 0b1100_0000 {
                    2
                } else if c & 0b1111_0000 == 0b1110_0000 {
                    3
                } else if c & 0b1111_1000 == 0b1111_0000 {
                    4
                } else {
                    panic!("Invalid UTF-8 leading byte");
                };
                return Err(LexerError::new(
                    LexerErrorKind::NonAsciiChar,
                    self.pos..self.pos + length,
                ));
            }
            c => {
                return Err(LexerError::new(
                    LexerErrorKind::InvalidToken(*c as char),
                    self.pos..self.pos + 1,
                ))
            }
        };
        self.read_next();
        Ok(Token { kind, span: 0..1 })
    }

    /// Increments the position until the next character to be read is not whitespace
    /// If it finds a newline returns [`Token::Newline`].
    /// The next character will be a non whitespace token.
    fn skip_whitespace(&mut self) -> Option<Token> {
        while let Some(curr) = self.peek() {
            if *curr == 0xA {
                self.read_next();
                return Some(Token::new(TokenKind::Newline, self.pos - 1..self.pos));
            } else if !curr.is_ascii_whitespace() {
                return None;
            }
            self.read_next();
        }
        None
    }

    /// Returns the byte that would be read next without altering the state of the Lexer.
    fn peek(&self) -> Option<&u8> {
        if self.pos >= self.input.len() {
            None
        } else {
            Some(&self.input[self.pos])
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
    fn read_string(&mut self) -> Result<Token, LexerError> {
        let start = self.pos;
        let mut string = String::new();
        let mut escaped = false;
        // skip the " that starts the string
        assert_eq!(self.read_next(), Some(b'"'));
        while let Some(c) = self.peek() {
            if escaped {
                escaped = false;
                //TODO: special chars like \n
                string.push(*c as char);
                self.read_next();
                continue;
            } else if *c == b'"' {
                break;
            } else if *c == b'\\' {
                escaped = true;
                self.read_next();
                continue;
            }
            string.push(*c as char);
            self.read_next();
        }
        // Next char should be '"' because the next token will be a closing quote,
        // if it's not the string is not closed and it's an error
        if self.read_next() != Some(b'"') {
            return Err(LexerError::new(
                LexerErrorKind::ExpectedStringEnd,
                start..self.pos,
            ));
        }
        Ok(Token::new(TokenKind::String(string), start..self.pos))
    }

    /// Reads an ident, keeps going until an ascii whitespace character is found
    fn read_ident_or_instruction(&mut self) -> Result<Token, LexerError> {
        let mut string = String::new();
        let start = self.pos;
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            string.push(*c as char);
            self.read_next();
        }
        let span = start..self.pos;
        // try parsing the string as an instruction, if invalid return as ident
        if let Ok(instruction) = string.parse::<InstructionKind>() {
            Ok(Token::new(TokenKind::Instruction(instruction), span))
        } else {
            Ok(Token::new(TokenKind::Ident(string), span))
        }
    }

    fn read_directive(&mut self) -> Result<Token, ()> {
        let mut string = String::new();
        let start = self.pos;
        // skip the `.`
        assert_eq!(self.read_next(), Some(b'.'));
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            string.push(*c as char);
            self.read_next();
        }
        // try parsing the string as a directive, if invalid reset the position and return error
        if let Ok(directive) = string.parse::<Directive>() {
            Ok(Token::new(TokenKind::Directive(directive), start..self.pos))
        } else {
            self.pos = start;
            Err(())
        }
    }

    fn read_number(&mut self) -> Result<Token, LexerError> {
        let start = self.pos;
        let mut string = String::new();
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            string.push(*c as char);
            self.read_next();
        }
        // parse the number with the correct radia based on the prefix
        let res = if string.starts_with("0x") {
            i16::from_str_radix(string.strip_prefix("0x").unwrap(), 16)
        } else if string.starts_with("0b") {
            i16::from_str_radix(string.strip_prefix("0b").unwrap(), 2)
        } else if string.starts_with("0o") {
            i16::from_str_radix(string.strip_prefix("0o").unwrap(), 8)
        } else {
            string.parse::<i16>()
        };
        let span = start..self.pos;
        match res {
            Ok(num) => Ok(Token::new(TokenKind::Number(num), span)),
            Err(err) => match err.kind() {
                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                    Err(LexerError::new(LexerErrorKind::NumberOutOfRange, span))
                }
                _ => Err(LexerError::new(LexerErrorKind::NumberParseError, span)),
            },
        }
    }

    /// Reads a register starting from a dollar sign, returning a [`Token::Register`], containing the representation of the
    /// register following the $
    fn read_register(&mut self) -> Result<Token, LexerError> {
        let start = self.pos;
        let mut chars = Vec::new();
        // skip the $ itself
        assert_eq!(self.read_next(), Some(b'$'));
        while let Some(c) = self.peek() {
            if !c.is_ascii_alphanumeric() {
                break;
            }
            chars.push(*c as char);
            self.read_next();
        }
        let span = start..self.pos;
        let register = match Register::try_from(chars.as_slice()) {
            Ok(reg) => reg,
            Err(err) => {
                return Err(LexerError {
                    kind: err.into(),
                    span,
                })
            }
        };
        Ok(Token::new(TokenKind::Register(register), span))
    }
}

#[cfg(test)]
mod tests {
    use crate::defs::directive::Directive;
    use crate::defs::register::RegisterParseError;
    use crate::defs::register::{RegisterName, RegisterPrefixedName};

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
            TokenKind::Directive(Directive::Data),
            TokenKind::Newline,
            TokenKind::Ident("x".into()),
            TokenKind::Colon,
            TokenKind::Directive(Directive::Word),
            TokenKind::Number(7),
            TokenKind::Newline,
            TokenKind::Ident("y".into()),
            TokenKind::Colon,
            TokenKind::Directive(Directive::Word),
            TokenKind::Number(3),
            TokenKind::Newline,
            TokenKind::Directive(Directive::Text),
            TokenKind::Newline,
            TokenKind::Ident("la".into()),
            TokenKind::Register(Register::PrefixedNumber(
                RegisterPrefixedName::new_unchecked('s', 0),
            )),
            TokenKind::Ident("x".into()),
            TokenKind::Newline,
            TokenKind::Ident("la".into()),
            TokenKind::Register(Register::PrefixedNumber(
                RegisterPrefixedName::new_unchecked('a', 0),
            )),
            TokenKind::Ident("mylabel".into()),
            TokenKind::Newline,
            TokenKind::Ident("li".into()),
            TokenKind::Register(Register::PrefixedNumber(
                RegisterPrefixedName::new_unchecked('v', 0),
            )),
            TokenKind::Number(4),
            TokenKind::Newline,
            TokenKind::Instruction("syscall".parse().unwrap()),
            TokenKind::Newline,
            TokenKind::Eof,
        ];
        let mut lexer = Lexer::new(input);
        for res in tokens.into_iter() {
            assert_eq!(lexer.next_token().unwrap().kind, res);
        }
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
    }
    #[test]
    fn read_strings() {
        let input = r#"data "inside string"
        out "inside \" escaped"
        double "inside\"some\"double"
        "#;
        let tokens = [
            TokenKind::Ident("data".into()),
            TokenKind::String("inside string".into()),
            TokenKind::Newline,
            TokenKind::Ident("out".into()),
            TokenKind::String("inside \" escaped".into()),
            TokenKind::Newline,
            TokenKind::Ident("double".into()),
            TokenKind::String("inside\"some\"double".into()),
            TokenKind::Newline,
        ];
        let mut lexer = Lexer::new(input);
        for res in tokens.into_iter() {
            assert_eq!(lexer.next_token().unwrap().kind, res);
        }
        assert_eq!(lexer.next_token().unwrap().kind, TokenKind::Eof);
        let mut lexer = Lexer::new("\"Open string");
        assert_eq!(
            lexer.next_token(),
            Err(LexerError::new(LexerErrorKind::ExpectedStringEnd, 0..12))
        );
    }
    #[test]
    fn lex() {
        let input = "lw $ra 4";
        let mut lexer = Lexer::new(input);
        let tokens = vec![
            TokenKind::Instruction("lw".parse().unwrap()),
            TokenKind::Register(Register::Name(RegisterName::Ra)),
            TokenKind::Number(4),
            TokenKind::Eof,
        ];
        //TODO: test spans
        assert_eq!(
            lexer
                .lex()
                .unwrap()
                .into_iter()
                .map(|t| t.kind)
                .collect::<Vec<_>>(),
            tokens
        );
        let input = "lw 4
iden
lw $error
test";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.lex(),
            Err(LexerError {
                kind: LexerErrorKind::Register(RegisterParseError::Other),
                span: 13..19
            })
        )
    }
    #[test]
    fn parse_numbers() {
        let mut lexer = Lexer::new("3 12 0x1f 0b1101 0o12");
        let tokens = vec![
            TokenKind::Number(3),
            TokenKind::Number(12),
            TokenKind::Number(31),
            TokenKind::Number(13),
            TokenKind::Number(10),
            TokenKind::Eof,
        ];
        assert_eq!(
            lexer
                .lex()
                .unwrap()
                .into_iter()
                .map(|t| t.kind)
                .collect::<Vec<_>>(),
            tokens
        );

        let strs = ["3a", "32768", "0x1h"];
        let mut errs = [
            LexerErrorKind::NumberParseError,
            LexerErrorKind::NumberOutOfRange,
            LexerErrorKind::NumberParseError,
        ]
        .into_iter();
        for s in strs {
            let mut lexer = Lexer::new(s);
            assert_eq!(
                lexer.lex(),
                Err(LexerError {
                    kind: errs.next().unwrap(),
                    span: 0..s.len()
                })
            );
        }
    }

    #[test]
    fn invalid_chars() {
        // 4, 3, 2 bytes respectively
        let strs = [" 😂 .text", "test €", "un è"];
        let ranges = [1..5, 5..8, 3..5];
        for (s, span) in strs.into_iter().zip(ranges.into_iter()) {
            let mut lexer = Lexer::new(s);
            assert_eq!(
                lexer.lex(),
                Err(LexerError {
                    kind: LexerErrorKind::NonAsciiChar,
                    span,
                })
            );
        }

        let mut lexer = Lexer::new("s~");
        assert_eq!(
            lexer.lex(),
            Err(LexerError {
                kind: LexerErrorKind::InvalidToken('~'),
                span: 1..2
            })
        )
    }
}
