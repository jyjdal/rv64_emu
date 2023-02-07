use std::ops::BitXor;
use crate::instruction::{decode, Instruction};
use crate::bus::*;
use crate::dram::*;
use crate::instruction::Instruction::*;

// CPU struct
pub struct Cpu {
    pub regs: [u64; 32],
    pub pc: u64,
    pub bus: Bus,
}

impl Cpu {
    pub fn new(binary: Vec<u8>) -> Self {
        let mut regs = [0; 32];
        // SP(Stack pointer) must be at the end of memory, so we set it first
        regs[2] = DRAM_BASE + DRAM_SIZE;

        Self {
            regs,
            pc: DRAM_BASE,
            bus: Bus::new(binary),
        }
    }

    // Dump register values
    pub fn dump_registers(&self) {
        let mut output = String::from("");
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        for i in (0..32).step_by(4) {
            output = format!(
                "{}\n{}",
                output,
                format!(
                    "x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x} x{:02}({})={:>#18x}",
                    i, abi[i], self.regs[i],
                    i + 1, abi[i + 1], self.regs[i + 1],
                    i + 2, abi[i + 2], self.regs[i + 2],
                    i + 3, abi[i + 3], self.regs[i + 3],
                )
            );
        }
        println!("{}", output);
    }

    // Load value from memory
    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        self.bus.load(addr, size)
    }

    // Store value to memory
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        self.bus.store(addr, size, value)
    }

    // Get an instruction
    pub fn fetch(&mut self) -> Result<u32, ()> {
        match self.bus.load(self.pc, 32) {
            Ok(inst) => {
                let inst = inst as u32;
                Ok(inst)
            }
            Err(_) => Err(())
        }
    }

    // Execute an instruction
    pub fn execute(&mut self, inst: u32) -> Result<(), ()> {
        let instruction = decode(inst);

        match instruction {
            Lb { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let addr = self.regs[rs1].wrapping_add(imm as i64 as u64);

                let val = self.load(addr, 8)?;
                self.regs[rd] = val as i8 as i64 as u64;
            }
            Lh { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let addr = self.regs[rs1].wrapping_add(imm as i64 as u64);

                let val = self.load(addr, 16)?;
                self.regs[rd] = val as i16 as i64 as u64;
            }
            Lw { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let addr = self.regs[rs1].wrapping_add(imm as i64 as u64);

                let val = self.load(addr, 32)?;
                self.regs[rd] = val as i32 as i64 as u64;
            }
            Ld { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let addr = self.regs[rs1].wrapping_add(imm as i64 as u64);

                let val = self.load(addr, 64)?;
                self.regs[rd] = val as i64 as u64;
            }
            Lbu { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let addr = self.regs[rs1].wrapping_add(imm as i64 as u64);

                let val = self.load(addr, 8)?;
                self.regs[rd] = val;
            }
            Lhu { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let addr = self.regs[rs1].wrapping_add(imm as i64 as u64);

                let val = self.load(addr, 16)?;
                self.regs[rd] = val;
            }
            Lwu { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let addr = self.regs[rs1].wrapping_add(imm as i64 as u64);

                let val = self.load(addr, 32)?;
                self.regs[rd] = val;
            }
            Addi { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                self.regs[rd] = self.regs[rs1].wrapping_add(imm as u64);
            }
            Slli { rd, rs1, shamt } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                self.regs[rd] = self.regs[rs1] << shamt;
            }
            Slti { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                self.regs[rd] = if (self.regs[rs1] as i64) < (imm as i64) {
                    1
                } else {
                    0
                };
            }
            Sltiu { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let imm = imm as u64;
                self.regs[rd] = if self.regs[rs1] < imm { 1 } else { 0 };
            }
            Xori { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let imm = imm as u64;
                self.regs[rd] = self.regs[rs1] ^ imm;
            }
            Srli { rd, rs1, shamt } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                self.regs[rd] = self.regs[rs1].wrapping_shr(shamt);
            }
            Srai { rd, rs1, shamt } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                self.regs[rd] = (self.regs[rs1] as i64).wrapping_shr(shamt) as u64;
            }
            Ori { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let imm = imm as u64;
                self.regs[rd] = self.regs[rs1] | imm;
            }
            Andi { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let imm = imm as u64;
                self.regs[rd] = self.regs[rs1] & imm;
            }
            Auipc { rd, imm } => {
                let rd = usize::from(rd);
                let imm = imm as i64 as u64;
                self.regs[rd] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            Addiw { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let imm = imm as u64;
                self.regs[rd] = self.regs[rs1].wrapping_add(imm) as i32 as i64 as u64;
            }
            Slliw { rd, rs1, shamt } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                self.regs[rd] = self.regs[rs1].wrapping_shl(shamt) as i32 as i64 as u64;
            }
            Srliw { rd, rs1, shamt } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                self.regs[rd] = self.regs[rs1].wrapping_shr(shamt) as i32 as i64 as u64;
            }
            Sraiw { rd, rs1, shamt } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                self.regs[rd] = self.regs[rs1].wrapping_shr(shamt) as i64 as u64;
            }
            Sb { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let addr = self.regs[rs1].wrapping_add(imm as u64);
                self.store(addr, 8, self.regs[rs2])?;
            }
            Sh { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let addr = self.regs[rs1].wrapping_add(imm as u64);
                self.store(addr, 16, self.regs[rs2])?;
            }
            Sw { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let addr = self.regs[rs1].wrapping_add(imm as u64);
                self.store(addr, 32, self.regs[rs2])?;
            }
            Sd { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let addr = self.regs[rs1].wrapping_add(imm as u64);
                self.store(addr, 64, self.regs[rs2])?;
            }
            Add { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            Sub { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]);
            }
            Sll { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let shamt = (self.regs[rs2] & 0x3f) as u64 as u32;
                self.regs[rd] = self.regs[rs1].wrapping_shl(shamt);
            }
            Slt { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                self.regs[rd] = if (self.regs[rs1] as i64) < (self.regs[rs2] as i64) { 1 } else { 0 };
            }
            Sltu { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                self.regs[rd] = if self.regs[rs1] < self.regs[rs2] { 1 } else { 0 };
            }
            Xor { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                self.regs[rd] = self.regs[rs1] ^ self.regs[rs2];
            }
            Srl { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let shamt = (self.regs[rs2] & 0x3f) as u64 as u32;
                self.regs[rd] = self.regs[rs1].wrapping_shr(shamt);
            }
            Sra { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let shamt = (self.regs[rs2] & 0x3f) as u64 as u32;
                self.regs[rd] = (self.regs[rs1] as i64).wrapping_shr(shamt) as u64;
            }
            Or { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                self.regs[rd] = self.regs[rs1] | self.regs[rs2];
            }
            And { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                self.regs[rd] = self.regs[rs1] & self.regs[rs2];
            }
            Lui { rd, imm } => {
                let rd = usize::from(rd);
                self.regs[rd] = imm as i64 as u64;
            }
            Addw { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]) as i32 as i64 as u64;
            }
            Subw { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                self.regs[rd] = self.regs[rs1].wrapping_sub(self.regs[rs2]) as i32 as u64;
            }
            Sllw { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let shamt = (self.regs[rs2] & 0x3f) as u64 as u32;
                self.regs[rd] = (self.regs[rs1] as u32).wrapping_shl(shamt) as i32 as u64;
            }
            Srlw { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let shamt = (self.regs[rs2] & 0x3f) as u64 as u32;
                self.regs[rd] = (self.regs[rs1] as u32).wrapping_shr(shamt) as i32 as u64;
            }
            Sraw { rd, rs1, rs2 } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let shamt = (self.regs[rs2] & 0x3f) as u64 as u32;
                self.regs[rd] = ((self.regs[rs1] as i32) >> (shamt as i32)) as u64;
            }
            Beq { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let imm = imm as u64;
                if self.regs[rs1] == self.regs[rs2] {
                    self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                }
            }
            Bne { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let imm = imm as u64;
                if self.regs[rs1] != self.regs[rs2] {
                    self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                }
            }
            Blt { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let imm = imm as u64;
                if (self.regs[rs1] as i64) < (self.regs[rs2] as i64) {
                    self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                }
            }
            Bge { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let imm = imm as u64;
                if (self.regs[rs1] as i64) >= (self.regs[rs2] as i64) {
                    self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                }
            }
            Bltu { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let imm = imm as u64;
                if self.regs[rs1] < self.regs[rs2] {
                    self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                }
            }
            Bgeu { rs1, rs2, imm } => {
                let rs1 = usize::from(rs1);
                let rs2 = usize::from(rs2);
                let imm = imm as u64;
                if self.regs[rs1] >= self.regs[rs2] {
                    self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
                }
            }
            Jalr { rd, rs1, imm } => {
                let rd = usize::from(rd);
                let rs1 = usize::from(rs1);

                let t = self.pc;
                let imm = imm as i64 as u64;
                // Don't add 4 because pc already moved on.
                self.pc = (self.regs[rs1].wrapping_add(imm)) & !1;
                self.regs[rd] = t;
            }
            Jal { rd, imm } => {
                let rd = usize::from(rd);
                self.regs[rd] = self.pc;
                let imm = imm as i64 as u64;
                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            _ => {
                dbg!(format!("Invalid instruction: {:?}.", inst));
                return Err(());
            }
        }

        Ok(())
    }
}
