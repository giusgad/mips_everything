use crate::errors::LexerErrorKind;
use strum::EnumString;
use thiserror::Error;

#[derive(Debug, Error, Eq, PartialEq)]
#[error("Couldn't parse \"{reg}\", {kind}.")]
pub struct RegisterParseError {
    kind: RegisterParseErrorKind,
    reg: String,
}
#[derive(Debug, Error, Eq, PartialEq)]
enum RegisterParseErrorKind {
    #[error("\"{0}\" is not a valid register prefix, must be one of 'v','a','t','s','k'")]
    InvalidPrefix(char),
    #[error(
        "\"{0}\" is not a valid register index, must be $a0-$a3,$t0-$t9,$s0-$s7,$k0-$k1 or $v0-$v1"
    )]
    InvalidIndex(String),
}

#[derive(Debug, PartialEq, Eq)]
/// Identifies a valid register in the CPU
pub enum Register {
    /// Register Identified directly by number
    Number(u8),
    /// Register Identified by letter+number like `$s1` or `$t1`
    PrefixedNumber(RegisterPrefixedName),
    /// Registers Identified by name like as `$ra`
    Name(RegisterName),
}

impl TryFrom<&[char]> for Register {
    type Error = LexerErrorKind;

    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RegisterPrefixedName {
    /// The prefix of the register alias, can be one of 'v','a','t','s','k'
    prefix: char,
    /// The number after the prefix
    index: u8,
}

impl TryFrom<&[char]> for RegisterPrefixedName {
    type Error = RegisterParseErrorKind;

    fn try_from(chars: &[char]) -> Result<Self, Self::Error> {
        // there must to be at least 2 chars: ['s','7']
        assert!(chars.len() >= 2);
        // if there is more than 2 chars, the number can't be valid since all register numbers only
        // have one digit (max is 9 for $t9)
        if chars.len() > 2 {
            return Err(RegisterParseErrorKind::InvalidIndex(String::from_iter(
                &chars[1..],
            )));
        }
        let mut res = RegisterPrefixedName {
            prefix: ' ',
            index: 0,
        };
        // get the prefix of the register
        res.prefix = match chars[0] {
            c @ ('v' | 'a' | 't' | 's' | 'k') => c,
            c => return Err(RegisterParseErrorKind::InvalidPrefix(c)),
        };
        // try to parse the index
        if let Some(index) = chars[1].to_digit(10) {
            // get the max index the register can have
            // there e.g. there is only $v0-$v1, $a0-$a3...
            let max = match res.prefix {
                'v' => 1,
                'a' => 3,
                't' => 9,
                's' => 7,
                'k' => 1,
                _ => unreachable!(),
            };
            if index > max {
                return Err(RegisterParseErrorKind::InvalidIndex(chars[1].into()));
            }
            res.index = index as u8;
        } else {
            return Err(RegisterParseErrorKind::InvalidIndex(chars[1].into()));
        }
        Ok(res)
    }
}

#[derive(Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
/// register name
enum RegisterName {
    At,
    Gp,
    Sp,
    Fp,
    Ra,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn prefixed_name() {
        assert_eq!(
            RegisterPrefixedName::try_from(&['s', '3'] as &[char]),
            Ok(RegisterPrefixedName {
                prefix: 's',
                index: 3
            })
        );
    }
}
