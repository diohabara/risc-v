const MEMORY_CAPACITY: usize = 1024 * 16;
const CSR_CAPACITY: usize = 4096;

pub struct Cpu {
    x: [i32; 32],
    pc: u32,
    csr: [u32; CSR_CAPACITY],
    memory: [u8; MEMORY_CAPACITY],
}
enum Instruction {
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
    ECALL,
    EBREAK,
}

enum InstructionFormat {
    R,
    I,
    S,
    B,
    U,
    J,
    O, // Others
}

fn get_instruction_name(instruction: &Instruction) -> &'static str {
    match instruction {
        // Integer Computational Instructions
        Instruction::ADDI => "ADD",
        /// Integer Register-Immediate Instructions
        Instruction::SLTI => "SLTI",
        Instruction::SLTIU => "SLTIU",
        Instruction::ANDI => "ANDI",
        Instruction::ORI => "ORI",
        Instruction::XORI => "XORI",
        Instruction::SLLI => "SLLI",
        Instruction::SRLI => "SRLI",
        Instruction::SRAI => "SRAI",
        Instruction::LUI => "LUI",
        Instruction::AUIPC => "AUIPC",
        /// Integer Register-Register Operations
        Instruction::ADD => "ADD",
        Instruction::SLT => "SLT",
        Instruction::SLTU => "SLTU",
        Instruction::AND => "AND",
        Instruction::OR => "OR",
        Instruction::XOR => "XOR",
        Instruction::SLL => "SLL",
        Instruction::SRL => "SRL",
        Instruction::SUB => "SUB",
        Instruction::SRA => "SRA",
        // Control Transfer Instructions
        /// Unconditional Jumps
        Instruction::JAL => "JAL",
        Instruction::JALR => "JALR",
        /// Conditional Branches
        Instruction::BEQ => "BEQ",
        Instruction::BNE => "BNE",
        Instruction::BLT => "BLT",
        Instruction::BLTU => "BLTU",
        Instruction::BGE => "BGE",
        Instruction::BGEU => "BGEU",
        // Load and Store Instructions
        Instruction::LW => "LW",
        Instruction::LH => "LH",
        Instruction::LHU => "LHU",
        Instruction::LB => "LB",
        Instruction::LBU => "LBU",
        Instruction::SW => "SW",
        Instruction::SH => "SH",
        Instruction::SB => "SB",
        // Memory Model
        Instruction::FENCE => "FENCE",
        Instruction::FENCE_I => "FENCE_I",
        // Control and Status Register Instructions
        Instruction::CSRRW => "CSRRW",
        Instruction::CSRRS => "CSRRS",
        Instruction::CSRRC => "CSRRC",
        Instruction::CSRRWI => "CSRRWI",
        Instruction::CSRRSI => "CSRRSI",
        Instruction::CSRRCI => "CSRRCI",
        // Environment Call and Breakpoints
        Instruction::ECALL => "ECALL",
        Instruction::EBREAK => "EBREAK",
    }
}

fn get_instruction_format(instruction: &Instruction) -> InstructionFormat {
    match instruction {
        Instruction::BEQ
        | Instruction::BNE
        | Instruction::BLT
        | Instruction::BGE
        | Instruction::BLTU
        | Instruction::BGEU => InstructionFormat::B,
        Instruction::LB
        | Instruction::LH
        | Instruction::LW
        | Instruction::LBU
        | Instruction::LHU
        | Instruction::ADDI
        | Instruction::SLTI
        | Instruction::SLTIU
        | Instruction::JALR
        | Instruction::XORI
        | Instruction::ORI
        | Instruction::ANDI
        | Instruction::XORI
        | Instruction::CSRRC
        | Instruction::CSRRCI
        | Instruction::CSRRS
        | Instruction::CSRRSI
        | Instruction::CSRRW
        | Instruction::CSRRWI
        | Instruction::ECALL
        | Instruction::EBREAK => InstructionFormat::I,
        Instruction::JAL => InstructionFormat::J,
        Instruction::SLLI
        | Instruction::SRLI
        | Instruction::SRAI
        | Instruction::ADD
        | Instruction::SUB
        | Instruction::SLL
        | Instruction::SLT
        | Instruction::SLTU
        | Instruction::XOR
        | Instruction::SRA
        | Instruction::SRL
        | Instruction::OR
        | Instruction::AND => InstructionFormat::R,
        Instruction::SW | Instruction::SH | Instruction::SB => InstructionFormat::S,
        AUIPC | LUI => InstructionFormat::U,
        Instruction::FENCE | Instruction::FENCE_I => InstructionFormat::O,
    }
}

impl Cpu {
    pub fn new() -> Self {}
}
