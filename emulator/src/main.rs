#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InstKind {
    // Integer Computational Instructions
    /// Integer Register-Immediate Instructions
    ADDI,
    SLTI,
    SLTIU,
    ANDI,
    ORI,
    XORI,
    SLLI,
    SRLI,
    SRAI,
    LUI,
    AUIPC,
    /// Integer Register-Register Operations
    ADD,
    SLT,
    SLTU,
    AND,
    OR,
    XOR,
    SLL,
    SRL,
    SUB,
    SRA,
    // Control Transfer Instructions
    /// Unconditional Jumps
    JAL,
    JALR,
    /// Conditional Branches
    BEQ,
    BNE,
    BLT,
    BLTU,
    BGE,
    BGEU,
    // Load and Store Instructions
    LW,
    LH,
    LHU,
    LB,
    LBU,
    SW,
    SH,
    SB,
    // Memory Model
    FENCE,
    FENCE_I,
    // Control and Status Register Instructions
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,
    // Environment Call and Breakpoints
    PRIV,
}
fn main() {}
