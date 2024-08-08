/// represents a parsed MIPS program
pub struct Program {
    segments: Vec<Segment>,
    labels: Vec<String>,
    macros: Vec<String>,
}

enum StorableData {
    Double(u32),
    Float(u32),
    Int(u32),
    Word(u32),
    Half(u16),
}

struct Instruction {
    //TODO: instrction repr
}

enum Statement {
    Instruction(Instruction),
    Data(StorableData),
}

enum SegmentKind {
    Data,
    Text,
    Kdata,
    Ktext,
    Macro,
}
struct Segment {
    kind: SegmentKind,
    stmts: Vec<Statement>,
}
