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
        | Instruction::CSRRW
        | Instruction::CSRRS
        | Instruction::CSRRC
        | Instruction::CSRRWI
        | Instruction::CSRRSI
        | Instruction::CSRRCI
        | Instruction::ECALL
        | Instruction::EBREAK
        | Instruction::FENCE
        | Instruction::FENCE_I => InstructionFormat::I,
        Instruction::SW | Instruction::SH | Instruction::SB => InstructionFormat::S,
        Instruction::BEQ
        | Instruction::BNE
        | Instruction::BLT
        | Instruction::BGE
        | Instruction::BLTU
        | Instruction::BGEU => InstructionFormat::B,
        Instruction::LUI | Instruction::AUIPC => InstructionFormat::U,
        Instruction::JAL => InstructionFormat::J,
    }
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            x: [0; 32],
            pc: 0,
            csr: [0; CSR_CAPACITY],
            memory: [0; MEMORY_CAPACITY],
        }
    }

    pub fn run_test(&mut self, data: Vec<u8>) {
        for i in 0..data.len() {
            self.memory[i] = data[i];
        }
        self.pc = 0;
        loop {
            let terminate = match self.load_word(self.pc) {
                0x0000_0073 => true,
                _ => false,
            };
            self.tick();
            if terminate {
                match self.x[10] {
                    0 => println!("Test passed!"),
                    _ => println!("Test failed..."),
                };
                break;
            }
        }
    }

    pub fn tick(&mut self) {
        let word = self.fetch();
        let instruction = self.decode(word);
        println!("PC:{:08x}, Word:{:08x}, Inst:{}"
                 self.pc.wrapping_sub(4), word,
                 get_instruction_name(&instruction));
        self.operate(word, instruction);
    }

    fn fetch(&mut self) -> u32 {
        let word = self.load_word(self.pc);
        self.pc = self.pc.wrapping_add(4);
        word
    }

    fn load_word(&self, address: u32) -> u32 {
        ((self.memory[address as usize + 3] as u32) << 24)
            | ((self.memory[address as usize + 2] as u32) << 16)
            | ((self.memory[address as usize + 1] as u32) << 8)
            | (self.memory[address as usize])
    }

    fn load_halfword(&self, address: u32) -> u16 {
        ((self.memory[address as usize + 1] as u32) << 8) | (self.memory[address as usize])
    }

    fn load_byte(&self, address: u32) -> u8 {
        (self.memory[address as usize])
    }

    fn store_word(&self, address: u32, value: u32) {
        // 0xff = 1111_1111
        self.memory[address as usize] = (value & 0xff) as u8;
        self.memory[address as usize + 1] = ((value >> 8) & 0xff) as u8;
        self.memory[address as usize + 2] = ((value >> 16) & 0xff) as u8;
        self.memory[address as usize + 3] = ((value >> 24) & 0xff) as u8;
    }

    fn store_halfword(&self, address: u32, value: u16) {
        self.memory[address as usize] = (value & 0xff) as u8;
        self.memory[address as usize + 1] = ((value >> 8) & 0xff) as u8;
    }

    fn store_byte(&self, address: u32, value: u8) {
        self.memory[address as usize] = value;
    }

    // look at P130
    fn decode(&self, word: u32) -> Instruction {
        let opcode = word & 0x7f; // [6:0]
        let funct3 = (word >> 12) & 0x7; // [14:12]
        let funct7 = (word >> 25) & 0x7f; // [31:25]
        let imm12 = (word >> 20) & 0x7ff; // [31:20]

        // R type
        if opcode == 0x13 {
            return match funct3 {
                0 => Instruction::SLLI,
                5 => match funct7 {
                    0x00 => Instruction::SRLI,
                    0x20 => Instruction::SRAI,
                },
            };
        }
        if opcode == 0x33 {
            return match funct3 {
                0 => match funct7 {
                    0x00 => Instruction::ADD,
                    0x20 => Instruction::SUB,
                },
                1 => Instruction::SLL,
                2 => Instruction::SLT,
                3 => Instruction::SLTU,
                4 => Instruction::XOR,
                5 => match funct7 {
                    0x00 => Instruction::SRL,
                    0x20 => Instruction::SRA,
                },
                6 => Instruction::OR,
                7 => Instruction::AND,
            };
        }
        // I type
        if opcode == 0x67 {
            return Instruction::JALR;
        }
        if opcode == 0x03 {
            return match funct3 {
                0 => Instruction::LB,
                1 => Instruction::LH,
                2 => Instruction::LW,
                4 => Instruction::LBU,
                5 => Instruction::LHU,
                _ => {
                    error_funct3(funct3);
                }
            };
        }
        if opcode == 0x13 {
            return match funct3 {
                0 => Instruction::ADDI,
                2 => Instruction::SLTI,
                3 => Instruction::SLTIU,
                4 => Instruction::XORI,
                6 => Instruction::ORI,
                7 => Instruction::ANDI,
                _ => error_funct3(funct3),
            };
        }
        if opcode == 0x73 {
            return match funct3 {
                0 => match imm12 {
                    0 => Instruction::ECALL,
                    1 => Instruction::EBREAK,
                },
                1 => Instruction::CSRRW,
                2 => Instruction::CSRRS,
                3 => Instruction::CSRRC,
                5 => Instruction::CSRRWI,
                6 => Instruction::CSRRSI,
                7 => Instruction::CSRRCI,
                _ => error_funct3(funct3),
            };
        }
        if opcode == 0x0f {
            return match funct3 {
                0 => Instruction::FENCE,
                1 => Instruction::FENCE_I,
                _ => error_funct3(funct3),
            };
        }
        // S type
        if opcode == 0x23 {
            return match funct3 {
                0 => Instruction::SB,
                1 => Instruction::SH,
                2 => Instruction::SW,
            };
        }
        // B type
        if opcode == 0x63 {
            return match funct3 {
                0 => Instruction::BEQ,
                1 => Instruction::BNE,
                4 => Instruction::BLT,
                5 => Instruction::BGE,
                6 => Instruction::BLTU,
                7 => Instruction::BGEU,
                _ => error_funct3(funct3),
            };
        }
        // U type
        if opcode == 0x37 {
            return Instruction::LUI;
        }
        if opcode == 0x17 {
            return Instruction::AUIPC;
        }
        // J type
        if opcode == 0x6f {
            return Instruction::JAL;
        }
    }

    fn operate(&mut self, word: u32, instruction: Instruction) {
        let instruction_format = get_instruction_format(&instruction);
        match instruction_format {
            InstructionFormat::R => {}
            InstructionFormat::I => {}
            InstructionFormat::S => {}
            InstructionFormat::B => {}
            InstructionFormat::U => {}
            InstructionFormat::J => {}
        }
    }
}

fn error_funct3(funct3: u32) {
    println!("Unknown funct3: RV32I does not support {03:b}...", funct3);
    panic!();
}
