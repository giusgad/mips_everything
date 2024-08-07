use strum::EnumString;

#[derive(Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "snake_case")]
/// A mips directive like `.text`,`.align`,`.half`...
/// Note that the `.` dot is not represented in the tokens,
/// a directive token implicitly contains the dot.
pub(crate) enum Directive {
    /// Align the next data item on the specified byte boundary
    Align,
    /// Store the string in the data segment without null terminator
    Ascii,
    /// Store the string in the data segment with null terminator
    Asciiz,
    /// Store the following values as bytes
    Byte,
    /// Begin the data segment
    Data,
    /// Store the following values as double precision floating point numbers
    Double,
    /// End macro definition
    EndMacro,
    /// Substitute the second operand for the first in the program (like C's #define)
    Eqv,
    /// Declare the label and byte length as a global data field
    Extern,
    /// Store the following values as single precision floating point numbers
    Float,
    /// Set the following labels as global
    Globl,
    /// Store the following values as half words (16 bit)
    Half,
    /// Includes the contents of a file, specified as path between quotes
    // TODO: parse this
    Include,
    /// Begin kdata segment
    Kdata,
    /// Begin ktext segment
    Ktext,
    /// Begin macro definition
    Macro,
    /// Reserve the specified amount of bytes in the data segment
    Space,
    /// Begin the text segment
    Text,
    /// Store the following values as words (32 bit)
    Word,
}

#[cfg(test)]
mod tests {
    use crate::defs::directive::Directive;

    use super::Directive::*;
    #[test]
    fn directive_str_parse() {
        const ERR: Result<Directive, strum::ParseError> = Err(strum::ParseError::VariantNotFound);
        #[rustfmt::skip]
        let strs = ["kdata", "end_macro", "endmacro", "EndMacro", "asciiz","ascii", "include", "word"];
        #[rustfmt::skip]
        let dirs = [Ok(Kdata), Ok(EndMacro), ERR, ERR, Ok(Asciiz), Ok(Ascii), Ok(Include), Ok(Word)];
        for (s, d) in strs.into_iter().zip(dirs.into_iter()) {
            assert_eq!(s.parse::<Directive>(), d);
        }
    }
}
