#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InstKind {
    // Integer Cmputational Instructions
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
    ADDI,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RegKind {
    REG0,
    REG1,
    REG2,
    REG3,
    REG4,
    REG5,
    REG6,
    REG7,
}
static mut reg: [i32; 8] = [0; 8];
static mut rom: [i32; 256] = [0; 256];
static mut ram: [i32; 256] = [0; 256];
fn main() {
    let mut pc: i32 = 0;
    let mut inst_reg: i32 = 0;
    let flag_eq: bool = false;

    assemble();

    loop {
        println!(
            "{}, {}, {}, {}, {}, {}",
            &pc, &inst_reg, &reg[0], &reg[1], &reg[2], &reg[3]
        );

        if op_code(inst_reg) == InstKind::HLT {
            break;
        }

        pc += 1;

        match op_code(inst_reg) {
            InstKind::MOV => reg[op_regA(inst_reg)] = reg[op_regB(inst_reg)],
            InstKind::ADD => {
                reg[op_regA(inst_reg)] = reg[op_regA(inst_reg)] + reg[op_regB(inst_reg)]
            }
            InstKind::SUB => {
                reg[op_regA(inst_reg)] = reg[op_regA(inst_reg)] - reg[op_regB(inst_reg)]
            }
            InstKind::AND => {
                reg[op_regA(inst_reg)] = reg[op_regA(inst_reg)] & reg[op_regB(inst_reg)]
            }
            InstKind::OR => {
                reg[op_regA(inst_reg)] = reg[op_regA(inst_reg)] | reg[op_regB(inst_reg)]
            }
            InstKind::SL => reg[op_regA(inst_reg)] = reg[op_regA(inst_reg)] << 1,
            InstKind::SR => reg[op_regA(inst_reg)] = reg[op_regA(inst_reg)] >> 1,
            InstKind::SRA => {
                reg[op_regA(inst_reg)] =
                    reg[op_regA(inst_reg)] & 0x8000 | reg[op_regB(inst_reg)] >> 1
            }
            InstKind::LDL => {
                reg[op_regA(inst_reg)] =
                    (reg[op_regA(inst_reg)] & 0xff00) | (reg[op_regB(inst_reg)] & 0x00ff)
            }
            InstKind::LDH => {
                reg[op_regA(inst_reg)] =
                    (reg[op_regA(inst_reg)] << 8) | (reg[op_regB(inst_reg)] & 0x00ff)
            }
            InstKind::CMP => {
                if reg[op_regA(inst_reg)] == reg[op_regB(inst_reg)] {
                    flag_eq = true;
                } else {
                    flag_eq = false;
                }
            }
            InstKind::JE => {
                if reg[op_regA(inst_reg)] == 1 {
                    pc = op_addr(inst_reg);
                }
            }
            InstKind::JMP => {
                if flag_eq {
                    pc = op_addr(inst_reg);
                }
            }
            InstKind::LD => {
                reg[op_regA(inst_reg)] = ram[op_addr(inst_reg)];
            }
            InstKind::ST => {
                reg[op_addr(inst_reg)] = reg[op_regA(inst_reg)];
            }
        }
    }

    println!("result = {}", &ram[64]);
}

fn assemble() {
    rom[0] = ldh(RegKind::REG0, 0);
    rom[1] = ldl(RegKind::REG0, 0);
    rom[2] = ldh(RegKind::REG1, 0);
    rom[3] = ldl(RegKind::REG1, 1);
    rom[4] = ldh(RegKind::REG2, 0);
    rom[5] = ldl(RegKind::REG2, 0);
    rom[6] = ldh(RegKind::REG3, 0);
    rom[7] = ldl(RegKind::REG3, 0);
    rom[8] = add(RegKind::REG2, RegKind::REG1);
    rom[9] = add(RegKind::REG0, RegKind::REG2);
    rom[10] = st(RegKind::REG0, 64);
    rom[11] = cmp(RegKind::REG2, RegKind::REG3);
    rom[12] = je(14);
    rom[13] = jmp(8);
    rom[14] = hlt();
}

fn mov(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::MOV << 11) | (reg_a << 8) | (reg_b << 5);
}

fn add(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::ADD << 11) | (reg_a << 8) | (reg_b << 5);
}

fn sub(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::SUB << 11) | (reg_a << 8) | (reg_b << 5);
}

fn and(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::AND << 11) | (reg_a << 8) | (reg_b << 5);
}

fn or(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::OR << 11) | (reg_a << 8) | (reg_b);
}

fn sl(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::SL << 11) | (reg_a << 8);
}

fn sr(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::SR << 11) | (reg_a << 8);
}

fn sra(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::SRA << 11) | (reg_a << 8);
}

fn ldl(reg_a: i32, ival: i32) -> i32 {
    return (InstKind::LDL << 11) | (reg_a << 8) | (ival & 0x00ff);
}

fn ldh(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::LDL << 11) | (reg_a << 8) | (ival & 0x00ff);
}

fn cmp(reg_a: i32, reg_b: i32) -> i32 {
    return (InstKind::CMP << 11) | (reg_a << 8) | (reg_b << 5);
}

fn je(addr: i32) -> i32 {
    return (InstKind::JE << 11) | (addr & 0x00ff);
}

fn jmp(addr: i32) -> i32 {
    return (InstKind::JMP << 11) | (addr & 0x00ff);
}

fn ld(reg_a: i32, addr: i32) -> i32 {
    return (InstKind::LD << 11) | (reg_a << 8) | (addr & 0x00ff);
}

fn st(reg_a: i32, addr: i32) -> i32 {
    return (InstKind::ST << 11) | (reg_a << 8) | (addr & 0x00ff);
}
fn hlt() {
    return InstKind::HLT << 11;
}
fn op_code(inst_reg: i32) -> i32 {
    return inst_reg >> 11;
}
fn op_regA(inst_reg: i32) -> i32 {
    return (inst_reg >> 5) & 0x0007;
}
fn op_regB(inst_reg: i32) -> i32 {
    return (inst_reg >> 5) & 0x0007;
}
fn op_data(inst_reg: i32) -> i32 {
    return inst_reg & 0x00ff;
}
fn op_addr(inst_reg: i32) -> i32 {
    return inst_reg & 0x00ff;
}
