use std::str::FromStr;
use strum::EnumString;
use thiserror::Error;

const REG_MUST_BE: &str = "must be $0-$31 or $a0-$a3,$t0-$t9,$s0-$s7,$k0-$k1,$v0-$v1";

#[derive(Debug, Error, Eq, PartialEq)]
#[error("Couldn't parse \"{reg}\", {kind}.")]
pub struct RegisterParseError {
    kind: RegisterParseErrorKind,
    reg: String,
}
#[derive(Debug, Error, Eq, PartialEq)]
pub enum RegisterParseErrorKind {
    #[error("\"{0}\" is not a valid register prefix, must be one of 'v','a','t','s','k'")]
    InvalidPrefix(char),
    #[error("\"{0}\" is not a valid register index, {REG_MUST_BE}")]
    InvalidIndex(String),
    #[error("Register number is out of range, {REG_MUST_BE}")]
    OutOfRange(u8),
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
    type Error = RegisterParseError;

    fn try_from(value: &[char]) -> Result<Self, Self::Error> {
        let reg_string = String::from_iter(value);
        // try to parse the register from name
        if let Ok(name) = RegisterName::from_str(&reg_string) {
            return Ok(Register::Name(name));
        }
        // try to parse the register as a number $0-$31
        if let Ok(num) = reg_string.parse::<u8>() {
            if num >= 32 {
                return Err(RegisterParseError {
                    kind: RegisterParseErrorKind::OutOfRange(num),
                    reg: reg_string,
                });
            }
            return Ok(Register::Number(num));
        }
        // try to parse the register as a prefixed alias like $v0,$s3...
        match RegisterPrefixedName::try_from(value) {
            Ok(reg) => Ok(Register::PrefixedNumber(reg)),
            Err(kind) => Err(RegisterParseError {
                kind,
                reg: reg_string,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RegisterPrefixedName {
    /// The prefix of the register alias, can be one of 'v','a','t','s','k'
    prefix: char,
    /// The number after the prefix
    index: u8,
}

impl RegisterPrefixedName {
    pub fn new_unchecked(prefix: char, index: u8) -> Self {
        //TODO: validation
        Self { prefix, index }
    }
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
                return Err(RegisterParseErrorKind::OutOfRange(index as u8));
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
pub enum RegisterName {
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
        let valids = [
            ("s7", 's', 7),
            ("v1", 'v', 1),
            ("a3", 'a', 3),
            ("k0", 'k', 0),
            ("t9", 't', 9),
        ];
        for (s, prefix, index) in valids {
            assert_eq!(
                RegisterPrefixedName::try_from(s.chars().collect::<Vec<char>>().as_slice()),
                Ok(RegisterPrefixedName { prefix, index })
            );
        }
        let out_of_range = [("s8", 8), ("k2", 2), ("a4", 4), ("v2", 2)];
        for (s, idx) in out_of_range {
            assert_eq!(
                RegisterPrefixedName::try_from(s.chars().collect::<Vec<char>>().as_slice()),
                Err(RegisterParseErrorKind::OutOfRange(idx))
            );
        }
        let more_errs = ["s9", "r3", "g3", "s12"];
        for s in more_errs {
            assert!(
                RegisterPrefixedName::try_from(s.chars().collect::<Vec<char>>().as_slice())
                    .is_err()
            )
        }
    }
    #[test]
    fn name() {
        let valids = [
            ("at", RegisterName::At),
            ("gp", RegisterName::Gp),
            ("sp", RegisterName::Sp),
            ("fp", RegisterName::Fp),
            ("ra", RegisterName::Ra),
        ];
        for (s, res) in valids {
            assert_eq!(RegisterName::from_str(s), Ok(res))
        }
        let errs = ["At", "aT", "re", "FP"];
        for s in errs {
            assert!(RegisterName::from_str(s).is_err())
        }
    }
    #[test]
    fn all_kinds() {
        let valids = [
            (
                "t8",
                Register::PrefixedNumber(RegisterPrefixedName {
                    prefix: 't',
                    index: 8,
                }),
            ),
            ("8", Register::Number(8)),
            ("1", Register::Number(1)),
            ("31", Register::Number(31)),
            ("at", Register::Name(RegisterName::At)),
            ("sp", Register::Name(RegisterName::Sp)),
        ];
        for (s, res) in valids {
            assert_eq!(
                Register::try_from(s.chars().collect::<Vec<_>>().as_slice()),
                Ok(res)
            );
        }
        let errs = ["s9", "sd", "Ra", "t12", "32", "-1"];
        for s in errs {
            assert!(Register::try_from(s.chars().collect::<Vec<_>>().as_slice()).is_err());
        }
    }
}
