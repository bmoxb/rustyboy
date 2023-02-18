pub trait MemoryBankController {
    fn read8(&self, addr: u16) -> u8;
    fn write8(&mut self, _addr: u16, _value: u8) {}
}

pub struct NoMBC {
    rom: [u8; 0x8000],
}

impl NoMBC {
    pub fn new(data: &[u8]) -> Self {
        let mut rom = [0; 0x8000];
        for (addr, byte) in data.iter().enumerate() {
            rom[addr] = *byte;
        }
        NoMBC { rom }
    }
}

impl MemoryBankController for NoMBC {
    fn read8(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }
}
