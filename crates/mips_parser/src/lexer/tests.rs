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
        TokenKind::Whitespace,
        TokenKind::Directive(Directive::Word),
        TokenKind::Whitespace,
        TokenKind::Number(7),
        TokenKind::Newline,
        TokenKind::Ident("y".into()),
        TokenKind::Colon,
        TokenKind::Whitespace,
        TokenKind::Directive(Directive::Word),
        TokenKind::Whitespace,
        TokenKind::Number(3),
        TokenKind::Newline,
        TokenKind::Directive(Directive::Text),
        TokenKind::Newline,
        TokenKind::Ident("la".into()),
        TokenKind::Whitespace,
        TokenKind::Register(Register::PrefixedNumber(
            RegisterPrefixedName::new_unchecked('s', 0),
        )),
        TokenKind::Whitespace,
        TokenKind::Ident("x".into()),
        TokenKind::Newline,
        TokenKind::Ident("la".into()),
        TokenKind::Whitespace,
        TokenKind::Register(Register::PrefixedNumber(
            RegisterPrefixedName::new_unchecked('a', 0),
        )),
        TokenKind::Whitespace,
        TokenKind::Ident("mylabel".into()),
        TokenKind::Newline,
        TokenKind::Ident("li".into()),
        TokenKind::Whitespace,
        TokenKind::Register(Register::PrefixedNumber(
            RegisterPrefixedName::new_unchecked('v', 0),
        )),
        TokenKind::Whitespace,
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
        TokenKind::Whitespace,
        TokenKind::String("inside string".into()),
        TokenKind::Newline,
        TokenKind::Ident("out".into()),
        TokenKind::Whitespace,
        TokenKind::String("inside \" escaped".into()),
        TokenKind::Newline,
        TokenKind::Ident("double".into()),
        TokenKind::Whitespace,
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
        TokenKind::Whitespace,
        TokenKind::Register(Register::Name(RegisterName::Ra)),
        TokenKind::Whitespace,
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
        TokenKind::Whitespace,
        TokenKind::Number(12),
        TokenKind::Whitespace,
        TokenKind::Number(31),
        TokenKind::Whitespace,
        TokenKind::Number(13),
        TokenKind::Whitespace,
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

#[test]
fn comments() {
    let mut lexer = Lexer::new(
        "# Comment on line one
# some more comments on line two
# some more comments on line three
#",
    );
    assert_eq!(lexer.lex(), Ok(vec![Token::new(TokenKind::Eof, 91..92)]));
}
