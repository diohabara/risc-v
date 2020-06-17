#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InstKind {
    MOV,
    ADD,
    SUB,
    AND,
    OR,
    SL,
    SR,
    SRA,
    LDL,
    LDH,
    CMP,
    JE,
    JMP,
    LD,
    ST,
    HLT,
}
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
    let mut pc = 0;
    let mut inst_reg = 0;
    let flag_eq = false;

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
