use instruction_encoding_derive::InstructionEncoding;
use strum::EnumString;

use super::Bits;

/// The instruction format defines how the bits that compose it are interpreted.
/// The three possible variants contain documentation for the respective bit layout.
#[derive(Debug, PartialEq, Eq)]
pub enum InstructionFormat {
    /// opcode | rs | rt | rd | shamt | funct |
    /// 6 bits | 5  | 5  | 5  | 5     | 6     |
    R,
    /// opcode | rs | rt | const |
    /// 6 bits | 5  | 5  | 16    |
    I,
    /// opcode | pseudo-address |
    /// 6 bits | 26 bits        |
    J,
}

/// Information on a specific instruction
pub trait InstructionEncoding {
    fn format(&self) -> InstructionFormat;
    fn opcode(&self) -> Bits<6>;
    fn funct(&self) -> Option<Bits<6>>;
}

#[derive(Debug, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
/// All possible instructions
//TODO: pseudo-instructions
pub(crate) enum InstructionKind {
    /***** ARITHMETIC INSTRUCTIONS *****/
    /// Add Word
    // #[instruction(opcode=0b010010, format=J)]
    Add,
    /// Add Immediate Word
    Addi,
    /// Add Immediate Unsigned Word
    Addiu,
    /// Add unsigned word
    Addu,
    /// Count leading ones in word
    Clo,
    /// Count leading zeros in word
    Clz,
    /// Divide word
    Div,
    /// Divide unsigned word
    Divu,
    /// Multiply and add word to hi, lo
    Madd,
    /// Multiply and add unsigned word to hi, lo
    Maddu,
    /// Multiply and subtract word to hi, lo
    Msub,
    /// Multiply and subtract unsigned word to hi, lo
    Msubu,
    /// Multiply word to gpr
    Mul,
    /// Multiply word
    Mult,
    /// Multiply unsigned word
    Multu,
    /// Set on less than
    Slt,
    /// Set on less than immediate
    Slti,
    /// Set on less than immediate unsigned
    Sltiu,
    /// Set on less than unsigned
    Sltu,
    /// Subtract word
    Sub,
    /// Subtract unsigned word
    Subu,

    /***** BRANCH AND JUMP *****/
    /// Unconditional Branch
    B,
    /// Branch and link
    Bal,
    /// Branch on equal
    Beq,
    /// Branch on greater than or equal to zero
    Bgez,
    /// Branch on greater than or equal to zero and link
    Bgezal,
    /// Branch on greater than zero
    Bgtz,
    /// Branch on less than or equal to zero
    Blez,
    /// Branch on less than zero
    Bltz,
    /// Branch on less than zero and link
    Bbltzal,
    /// Branch on not equal
    Bne,
    /// Jump
    J,
    /// Jump and link
    Jal,
    /// Jump and link register
    Jalr,
    /// Jump register
    Jr,

    /***** CPU CONTROL *****/
    /// No Operation
    Nop,
    /// Superscalar No Operation
    Ssnop,

    /***** LOAD, STORE, AND MEMORY *****/
    /// Load Byte
    Lb,
    /// Load Byte Unsigned
    Lbu,
    /// Load Halfword
    Lh,
    /// Load Halfword Unsigned
    Lhu,
    /// Load Linked Word
    Ll,
    /// Load Word
    Lw,
    /// Load Word Left
    Lwl,
    /// Load Word Right
    Lwr,
    /// Prefetch
    Pref,
    /// Store Byte
    Sb,
    /// Store Conditional Word
    Sc,
    /// Store Doubleword
    Sd,
    /// Store Halfword
    Sh,
    /// Store Word
    Sw,
    /// Store Word Left
    Swl,
    /// Store Word Right
    Swr,
    /// Synchronize Shared Memory
    Sync,

    /***** LOGICAL INSTRUCTIONS *****/
    /// And
    And,
    /// And Immediate
    Andi,
    /// Load Upper Immediate
    Lui,
    /// Not Or
    Nor,
    /// Or
    Or,
    /// Or Immediate
    Ori,
    /// Exclusive Or
    Xor,
    /// Exclusive Or Immediate
    Xori,

    /***** MOVE INSTRUCTIONS *****/
    /// Move From HI Register
    Mfhi,
    /// Move From LO Register
    Mflo,
    /// Move Conditional on Floating Point False
    Movf,
    /// Move Conditional on Not Zero
    Movn,
    /// Move Conditional on Floating Point True
    Movt,
    /// Move Conditional on Zero
    Movz,
    /// Move To HI Register
    Mthi,
    /// Move To LO Register
    Mtlo,

    /***** SHIFT INSTRUCTIONS *****/
    /// Shift Word Left Logical
    Sll,
    /// Shift Word Left Logical Variable
    Sllv,
    /// Shift Word Right Arithmetic
    Sra,
    /// Shift Word Right Arithmetic Variable
    Srav,
    /// Shift Word Right Logical
    Srl,
    /// Shift Word Right Logical Variable
    Srlv,

    /***** TRAP INSTRUCTIONS *****/
    /// Breakpoint
    Break,
    /// System Call
    Syscall,
    /// Trap if Equal
    Teq,
    /// Trap if Equal Immediate
    Teqi,
    /// Trap if Greater or Equal
    Tge,
    /// Trap if Greater of Equal Immediate
    Tgei,
    /// Trap if Greater or Equal Immediate Unsigned
    Tgeiu,
    /// Trap if Greater or Equal Unsigned
    Tgeu,
    /// Trap if Less Than
    Tlt,
    /// Trap if Less Than Immediate
    Tlti,
    /// Trap if Less Than Immediate Unsigned
    Tltiu,
    /// Trap if Less Than Unsigned
    Tltu,
    /// Trap if Not Equal
    Tne,
    /// Trap if Not Equal Immediate
    Tnei,

    /***** PRIVILEGED INSTRUCTIONS *****/
    /// Perform Cache Operation
    Cache,
    /// Exception Return
    Eret,
    /// Move from Coprocessor 0
    Mfc0,
    /// Move to Coprocessor 0
    Mtc0,
    /// Probe TLB for Matching Entry
    Tlbp,
    /// Read Indexed TLB Entry
    Tlbr,
    /// Write Indexed TLB Entry
    Tlbwi,
    /// Write Random TLB Entry
    Tlbwr,
    /// Enter Standby Mode
    Wait,
    /// Debug Exception Return
    Deret,
    /// Software Debug Breakpoint
    Sdbbp,
}
