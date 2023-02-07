use std::fmt::format;
use crate::instruction::{decode, Instruction};
use crate::bus::*;
use crate::dram::*;

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
            Instruction::Addi { rd, rs1, imm } => {
                let rd: usize = rd.into();
                let rs1: usize = rs1.into();
                self.regs[rd] = self.regs[rs1].wrapping_add(imm as u64);
            }
            Instruction::Slli { rd, rs1, shamt } => {
                let rd: usize = rd.into();
                let rs1: usize = rs1.into();
                self.regs[rd] = self.regs[rs1] << shamt;
            }
            Instruction::Slti { rd, rs1, imm } => {
                let rd: usize = rd.into();
                let rs1: usize = rs1.into();
                self.regs[rd] = if (self.regs[rs1] as i64) < (imm as i64) {
                    1
                } else {
                    0
                };
            }
            Instruction::Add { rd, rs1, rs2 } => {
                let rd: usize = rd.into();
                let rs1: usize = rs1.into();
                let rs2: usize = rs2.into();
                self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
            }
            _ => {
                dbg!(format!("Invalid opcode: {:?}.", instruction));
                return Err(());
            }
        }

        Ok(())
    }
}
