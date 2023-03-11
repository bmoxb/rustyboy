#![allow(dead_code, unused_variables)]

pub struct MBC1 {
    rom: [u8; 0x80000],           // 512 KiB
    ram_banks: [[u8; 0x2000]; 4], // 32 KiB (4 banks)
}

impl MBC1 {
    pub fn new(data: &[u8]) -> Self {
        unimplemented!()
    }
}

impl super::MemoryBankController for MBC1 {
    fn mbc_name(&self) -> &str {
        "MBC1"
    }

    fn read8(&self, addr: u16) -> u8 {
        unimplemented!()
    }

    fn write8(&mut self, addr: u16, value: u8) {
        unimplemented!()
    }
}
