use crate::bus::*;

// Init memory as 128MB
pub const DRAM_SIZE: u64 = 1024 * 1024 * 128;

// Dram
#[derive(Debug)]
pub struct Dram {
    pub dram: Vec<u8>,
}

impl Dram {
    pub fn new(code: Vec<u8>) -> Self {
        let mut dram = vec![0; DRAM_SIZE as usize];
        dram.splice(..code.len(), code.iter().cloned());
        Self { dram }
    }

    // API for load memory, little endian
    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        match size {
            8 => Ok(self.load8(addr)),
            16 => Ok(self.load16(addr)),
            32 => Ok(self.load32(addr)),
            64 => Ok(self.load64(addr)),
            _ => Err(())
        }
    }

    // API for store memory, little endian
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        match size {
            8 => Ok(self.store8(addr, value)),
            16 => Ok(self.store16(addr, value)),
            32 => Ok(self.store32(addr, value)),
            64 => Ok(self.store64(addr, value)),
            _ => Err(())
        }
    }

    // Internal methods for load memory
    fn load8(&self, addr: u64) -> u64 {
        let addr = (addr - DRAM_BASE) as usize;
        self.dram[addr] as u64
    }

    fn load16(&self, addr: u64) -> u64 {
        let addr = (addr - DRAM_BASE) as usize;
        self.dram[addr] as u64
            | ((self.dram[addr + 1] as u64) << 8)
    }

    fn load32(&self, addr: u64) -> u64 {
        let addr = (addr - DRAM_BASE) as usize;
        return self.dram[addr] as u64
            | ((self.dram[addr + 1] as u64) << 8)
            | ((self.dram[addr + 2] as u64) << 16)
            | ((self.dram[addr + 3] as u64) << 24);
    }

    fn load64(&self, addr: u64) -> u64 {
        let addr = (addr - DRAM_BASE) as usize;
        self.dram[addr] as u64
            | ((self.dram[addr + 1] as u64) << 8)
            | ((self.dram[addr + 2] as u64) << 16)
            | ((self.dram[addr + 3] as u64) << 24)
            | ((self.dram[addr + 4] as u64) << 32)
            | ((self.dram[addr + 5] as u64) << 40)
            | ((self.dram[addr + 6] as u64) << 48)
            | ((self.dram[addr + 7] as u64) << 56)
    }

    // Internal methods for store memory
    fn store8(&mut self, addr: u64, value: u64) {
        let addr = (addr - DRAM_BASE) as usize;
        self.dram[addr] = (value & 0xff) as u8
    }

    fn store16(&mut self, addr: u64, value: u64) {
        let addr = (addr - DRAM_BASE) as usize;
        self.dram[addr] = (value & 0xff) as u8;
        self.dram[addr + 1] = ((value >> 8) & 0xff) as u8;
    }

    fn store32(&mut self, addr: u64, value: u64) {
        let addr = (addr - DRAM_BASE) as usize;
        self.dram[addr] = (value & 0xff) as u8;
        self.dram[addr + 1] = ((value >> 8) & 0xff) as u8;
        self.dram[addr + 2] = ((value >> 16) & 0xff) as u8;
        self.dram[addr + 3] = ((value >> 24) & 0xff) as u8;
    }

    fn store64(&mut self, addr: u64, value: u64) {
        let addr = (addr - DRAM_BASE) as usize;
        self.dram[addr] = (value & 0xff) as u8;
        self.dram[addr + 1] = ((value >> 8) & 0xff) as u8;
        self.dram[addr + 2] = ((value >> 16) & 0xff) as u8;
        self.dram[addr + 3] = ((value >> 24) & 0xff) as u8;
        self.dram[addr + 4] = ((value >> 32) & 0xff) as u8;
        self.dram[addr + 5] = ((value >> 40) & 0xff) as u8;
        self.dram[addr + 6] = ((value >> 48) & 0xff) as u8;
        self.dram[addr + 7] = ((value >> 56) & 0xff) as u8;
    }
}
