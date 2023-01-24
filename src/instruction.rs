use crate::register::Register;

#[derive(Debug)]
pub enum Instruction {
    Undefined,

    Addi { rd: Register, rs1: Register, imm: i32 },
    Slti { rd: Register, rs1: Register, imm: i32 },
    Sltiu { rd: Register, rs1: Register, imm: i32 },
    Xori { rd: Register, rs1: Register, imm: i32 },
    Ori { rd: Register, rs1: Register, imm: i32 },
    Andi { rd: Register, rs1: Register, imm: i32 },

    Slli { rd: Register, rs1: Register, shamt: u32 },
    Srli { rd: Register, rs1: Register, shamt: u32 },
    Srai { rd: Register, rs1: Register, shamt: u32 },

    Lb { rd: Register, rs1: Register, imm: i32 },
    Lh { rd: Register, rs1: Register, imm: i32 },
    Lw { rd: Register, rs1: Register, imm: i32 },
    Lbu { rd: Register, rs1: Register, imm: i32 },
    Lhu { rd: Register, rs1: Register, imm: i32 },
    Lwu { rd: Register, rs1: Register, imm: i32 },
    Ld { rd: Register, rs1: Register, imm: i32 },

    Fence { rd: Register, rs1: Register, imm: i32 },

    Jalr { rd: Register, rs1: Register, imm: i32 },

    Ecall,
    Ebreak,

    Add { rd: Register, rs1: Register, rs2: Register },
}

// Instruction type, see specification chapter 27: RV32/64G Instruction Set Listings
#[derive(Copy, Clone)]
pub enum InstType {
    // Type R: func7, rs2, rs1, func3, rd, opcode
    R,
    // Type I: imm[11:0], rs1, func3, rd, opcode
    I,
    // Type S: imm[11:5], rs2, rs1, func3, imm[4:0], opcode
    S,
    // Type B: imm[12|10:5] rs2 rs1 func3 imm[4:1|11] opcode
    B,
    // Type U: imm[31:12] rd opcode
    U,
    // Type J: imm[20|10:1|11|19:12] rd opcode
    J,
}

impl InstType {
    // Decode instruction with known instruction type
    pub fn decode(&self, inst: u32) -> Instruction {
        match self {
            InstType::I => {
                return InstType::decode_type_i(inst);
            }
            InstType::R => {
                return InstType::decode_type_r(inst);
            }
            InstType::S => {
                return Instruction::Undefined;
            }
            _ => {}
        }

        return Instruction::Undefined;
    }

    fn decode_type_i(inst: u32) -> Instruction {
        // Get opcode
        let opcode = inst & 0b1111111;

        // Decode base type fields
        let imm = (inst >> 20) & 0b1111_1111_1111;
        let rs1: Register = (((inst >> 15) & 0b1111_1) as usize).into();
        let func3 = (inst >> 12) & 0b111;
        let rd: Register = (((inst >> 7) & 0b1111_1) as usize).into();

        // Sign extend the immediate
        let imm = ((imm as i32) << 20) >> 20;

        return match opcode {
            0b0000011 => {
                match func3 {
                    0b000 => Instruction::Lb { rd, rs1, imm },
                    0b001 => Instruction::Lh { rd, rs1, imm },
                    0b010 => Instruction::Lw { rd, rs1, imm },
                    0b100 => Instruction::Lbu { rd, rs1, imm },
                    0b101 => Instruction::Lhu { rd, rs1, imm },
                    0b110 => Instruction::Lwu { rd, rs1, imm },
                    0b011 => Instruction::Ld { rd, rs1, imm },
                    _ => Instruction::Undefined
                }
            }
            0b0010011 => {
                // Extract shift fields
                let shamt = (imm & 0b111_111) as u32;
                let shiftop = (imm >> 6) & 0b111_111;

                match func3 {
                    0b000 => Instruction::Addi { rd, rs1, imm },
                    0b010 => Instruction::Slti { rd, rs1, imm },
                    0b011 => Instruction::Sltiu { rd, rs1, imm },
                    0b100 => Instruction::Xori { rd, rs1, imm },
                    0b110 => Instruction::Ori { rd, rs1, imm },
                    0b111 => Instruction::Andi { rd, rs1, imm },
                    0b001 => Instruction::Slli { rd, rs1, shamt },
                    0b101 if shiftop == 0 => Instruction::Srli { rd, rs1, shamt },
                    0b101 if shiftop == 0b010_000 => Instruction::Srai { rd, rs1, shamt },
                    _ => Instruction::Undefined
                }
            }
            0b0001111 => {
                match func3 {
                    0b000 => Instruction::Fence { rd, rs1, imm },
                    _ => Instruction::Undefined
                }
            }
            0b1100111 => {
                match func3 {
                    0b000 => Instruction::Jalr { rd, rs1, imm },
                    _ => Instruction::Undefined
                }
            }
            0b1110011 => {
                match func3 {
                    0b000 if imm == 0 && rs1 == Register::X0
                        && rd == Register::X0 => Instruction::Ecall,
                    0b000 if imm == 1 && rs1 == Register::X0
                        && rd == Register::X0 => Instruction::Ebreak,
                    _ => Instruction::Undefined
                }
            }
            _ => { Instruction::Undefined }
        };
    }

    fn decode_type_r(inst: u32) -> Instruction {
        // Get opcode
        let opcode = inst & 0b1111111;

        // Decode base type fields
        let func7 = (inst >> 25) & 0b111_1111;
        let rs2 = (((inst >> 20) & 0b11111) as usize).into();
        let rs1: Register = (((inst >> 15) & 0b1111_1) as usize).into();
        let func3 = (inst >> 12) & 0b111;
        let rd: Register = (((inst >> 7) & 0b1111_1) as usize).into();

        return match opcode {
            0b0110011 => {
                Instruction::Add { rd, rs1, rs2 }
            }
            _ => Instruction::Undefined
        };
    }

    fn decode_type_u(inst: u32) -> Instruction {
        unimplemented!()
    }
}

// Decode a 32-bit instruction to enum Instruction
pub fn decode(inst: u32) -> Instruction {
    let opcode = inst & 0b1111111;

    if let Some(typ) = &OPCODE_TYPE[opcode as usize] {
        typ.decode(inst)
    } else {
        Instruction::Undefined
    }
}

// Map instruction opcode to its instruction type
const OPCODE_TYPE: [Option<InstType>; 128] = [
    /* 0b0000000 */ None,
    /* 0b0000001 */ None,
    /* 0b0000010 */ None,
    /* 0b0000011 */ Some(InstType::I),
    /* 0b0000100 */ None,
    /* 0b0000101 */ None,
    /* 0b0000110 */ None,
    /* 0b0000111 */ None,
    /* 0b0001000 */ None,
    /* 0b0001001 */ None,
    /* 0b0001010 */ None,
    /* 0b0001011 */ None,
    /* 0b0001100 */ None,
    /* 0b0001101 */ None,
    /* 0b0001110 */ None,
    /* 0b0001111 */ Some(InstType::I),
    /* 0b0010000 */ None,
    /* 0b0010001 */ None,
    /* 0b0010010 */ None,
    /* 0b0010011 */ Some(InstType::I),
    /* 0b0010100 */ None,
    /* 0b0010101 */ None,
    /* 0b0010110 */ None,
    /* 0b0010111 */ Some(InstType::U),
    /* 0b0011000 */ None,
    /* 0b0011001 */ None,
    /* 0b0011010 */ None,
    /* 0b0011011 */ None,
    /* 0b0011100 */ None,
    /* 0b0011101 */ None,
    /* 0b0011110 */ None,
    /* 0b0011111 */ None,
    /* 0b0100000 */ None,
    /* 0b0100001 */ None,
    /* 0b0100010 */ None,
    /* 0b0100011 */ Some(InstType::S),
    /* 0b0100100 */ None,
    /* 0b0100101 */ None,
    /* 0b0100110 */ None,
    /* 0b0100111 */ None,
    /* 0b0101000 */ None,
    /* 0b0101001 */ None,
    /* 0b0101010 */ None,
    /* 0b0101011 */ None,
    /* 0b0101100 */ None,
    /* 0b0101101 */ None,
    /* 0b0101110 */ None,
    /* 0b0101111 */ None,
    /* 0b0110000 */ None,
    /* 0b0110001 */ None,
    /* 0b0110010 */ None,
    /* 0b0110011 */ Some(InstType::R),
    /* 0b0110100 */ None,
    /* 0b0110101 */ None,
    /* 0b0110110 */ None,
    /* 0b0110111 */ Some(InstType::U),
    /* 0b0111000 */ None,
    /* 0b0111001 */ None,
    /* 0b0111010 */ None,
    /* 0b0111011 */ None,
    /* 0b0111100 */ None,
    /* 0b0111101 */ None,
    /* 0b0111110 */ None,
    /* 0b0111111 */ None,
    /* 0b1000000 */ None,
    /* 0b1000001 */ None,
    /* 0b1000010 */ None,
    /* 0b1000011 */ None,
    /* 0b1000100 */ None,
    /* 0b1000101 */ None,
    /* 0b1000110 */ None,
    /* 0b1000111 */ None,
    /* 0b1001000 */ None,
    /* 0b1001001 */ None,
    /* 0b1001010 */ None,
    /* 0b1001011 */ None,
    /* 0b1001100 */ None,
    /* 0b1001101 */ None,
    /* 0b1001110 */ None,
    /* 0b1001111 */ None,
    /* 0b1010000 */ None,
    /* 0b1010001 */ None,
    /* 0b1010010 */ None,
    /* 0b1010011 */ None,
    /* 0b1010100 */ None,
    /* 0b1010101 */ None,
    /* 0b1010110 */ None,
    /* 0b1010111 */ None,
    /* 0b1011000 */ None,
    /* 0b1011001 */ None,
    /* 0b1011010 */ None,
    /* 0b1011011 */ None,
    /* 0b1011100 */ None,
    /* 0b1011101 */ None,
    /* 0b1011110 */ None,
    /* 0b1011111 */ None,
    /* 0b1100000 */ None,
    /* 0b1100001 */ None,
    /* 0b1100010 */ None,
    /* 0b1100011 */ Some(InstType::B),
    /* 0b1100100 */ None,
    /* 0b1100101 */ None,
    /* 0b1100110 */ None,
    /* 0b1100111 */ Some(InstType::I),
    /* 0b1101000 */ None,
    /* 0b1101001 */ None,
    /* 0b1101010 */ None,
    /* 0b1101011 */ None,
    /* 0b1101100 */ None,
    /* 0b1101101 */ None,
    /* 0b1101110 */ None,
    /* 0b1101111 */ Some(InstType::J),
    /* 0b1110000 */ None,
    /* 0b1110001 */ None,
    /* 0b1110010 */ None,
    /* 0b1110011 */ Some(InstType::I),
    /* 0b1110100 */ None,
    /* 0b1110101 */ None,
    /* 0b1110110 */ None,
    /* 0b1110111 */ None,
    /* 0b1111000 */ None,
    /* 0b1111001 */ None,
    /* 0b1111010 */ None,
    /* 0b1111011 */ None,
    /* 0b1111100 */ None,
    /* 0b1111101 */ None,
    /* 0b1111110 */ None,
    /* 0b1111111 */ None,
];
