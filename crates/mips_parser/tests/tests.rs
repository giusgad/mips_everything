use instruction_encoding_derive::InstructionEncoding;
use mips_parser::defs::instruction::{InstructionEncoding, InstructionFormat};
use mips_parser::defs::Bits;

#[test]
fn instruction_derive_macro() {
    #[derive(InstructionEncoding)]
    enum Instruction {
        #[instruction(0b011010, R)]
        Add,
        #[instruction(0b011011, J)]
        Jump,
        #[instruction(0b010000, I)]
        Addi,
    }
    let add = Instruction::Add;
    let addi = Instruction::Addi;
    let jump = Instruction::Jump;

    assert_eq!(add.opcode(), Bits::<6>::new(0b011010));
    assert_eq!(add.format(), InstructionFormat::R);
    assert_eq!(jump.opcode(), Bits::new(0b011011));
    assert_eq!(jump.format(), InstructionFormat::J);
    assert_eq!(addi.opcode(), Bits::new(0b010000));
    assert_eq!(addi.format(), InstructionFormat::I);
}
