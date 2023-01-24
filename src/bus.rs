use crate::dram::*;

// Dram start address, same as QEMU
pub const DRAM_BASE: u64 = 0x8000_0000;

// Bus
pub struct Bus {
    dram: Dram,
}

impl Bus {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            dram: Dram::new(code)
        }
    }

    // API for load memory
    pub fn load(&self, addr: u64, size: u64) -> Result<u64, ()> {
        if addr < DRAM_BASE {
            return Err(());
        }

        return self.dram.load(addr, size);
    }

    // API for store memory
    pub fn store(&mut self, addr: u64, size: u64, value: u64) -> Result<(), ()> {
        if addr >= DRAM_BASE {
            return self.dram.store(addr, size, value);
        }
        Err(())
    }
}
