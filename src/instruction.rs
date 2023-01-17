use crate::register::Register;

#[derive(Debug)]
pub enum Instruction {
    Undefined,
    // Opcode: 0b0010011
    Addi { rd: Register, rs1: Register, imm: i32 },
    Slti { rd: Register, rs1: Register, imm: i32 },
    Sltiu { rd: Register, rs1: Register, imm: i32 },
    Xori { rd: Register, rs1: Register, imm: i32 },
    Ori { rd: Register, rs1: Register, imm: i32 },
    Andi { rd: Register, rs1: Register, imm: i32 },
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
        let opcode = inst & 0b1111111;

        match self {
            InstType::I => {
                // Decode base type fields
                let imm = (inst >> 20) & 0b1111_1111_1111;
                let rs1: Register = (((inst >> 15) & 0b1111_1) as usize).into();
                let func3 = (inst >> 12) & 0b111;
                let rd: Register = (((inst >> 7) & 0b1111_1) as usize).into();

                // Sign extend the immediate
                let imm = ((imm as i32) << 20) >> 20;

                return match opcode {
                    0b0010011 => {
                        match func3 {
                            0b000 => Instruction::Addi { rd, rs1, imm },
                            0b010 => Instruction::Slti { rd, rs1, imm },
                            0b011 => Instruction::Sltiu { rd, rs1, imm },
                            0b100 => Instruction::Xori { rd, rs1, imm },
                            0b110 => Instruction::Ori { rd, rs1, imm },
                            0b111 => Instruction::Andi { rd, rs1, imm },
                            _ => Instruction::Undefined
                        }
                    }
                    _ => { Instruction::Undefined }
                };
            }
            _ => {}
        }

        return Instruction::Undefined;
    }
}

#[test]
fn test() {
    // Instruction 0x00000013 is Addi { rd: X0, rs1: X0, imm: 0 }.
    decode(19);
    panic!("Failed")
}

// Decode a 32-bit instruction to enum Instruction
fn decode(inst: u32) {
    let opcode = inst & 0b1111111;

    let decoded = if let Some(typ) = OPCODE_TYPE[opcode as usize] {
        typ.decode(inst)
    } else {
        Instruction::Undefined
    };

    println!("Instruction {:#010x} is {:?}.\n", inst, decoded);
}

// Map instruction opcode to its instruction type
const OPCODE_TYPE: [Option<InstType>; 128] = [
    /* 0b0000000 */ None,
    /* 0b0000001 */ None,
    /* 0b0000010 */ None,
    /* 0b0000011 */ None,
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
    /* 0b0001111 */ None,
    /* 0b0010000 */ None,
    /* 0b0010001 */ None,
    /* 0b0010010 */ None,
    /* 0b0010011 */ Some(InstType::I),
    /* 0b0010100 */ None,
    /* 0b0010101 */ None,
    /* 0b0010110 */ None,
    /* 0b0010111 */ None,
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
    /* 0b0100011 */ None,
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
    /* 0b0110011 */ None,
    /* 0b0110100 */ None,
    /* 0b0110101 */ None,
    /* 0b0110110 */ None,
    /* 0b0110111 */ None,
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
    /* 0b1100011 */ None,
    /* 0b1100100 */ None,
    /* 0b1100101 */ None,
    /* 0b1100110 */ None,
    /* 0b1100111 */ None,
    /* 0b1101000 */ None,
    /* 0b1101001 */ None,
    /* 0b1101010 */ None,
    /* 0b1101011 */ None,
    /* 0b1101100 */ None,
    /* 0b1101101 */ None,
    /* 0b1101110 */ None,
    /* 0b1101111 */ None,
    /* 0b1110000 */ None,
    /* 0b1110001 */ None,
    /* 0b1110010 */ None,
    /* 0b1110011 */ None,
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
