pub struct RomOnly {
    rom: [u8; 0x8000],
}

impl RomOnly {
    pub fn new(data: &[u8]) -> Self {
        let mut rom = [0; 0x8000];
        for (addr, byte) in data.iter().enumerate() {
            rom[addr] = *byte;
        }
        RomOnly { rom }
    }
}

impl super::MemoryBankController for RomOnly {
    fn read8(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write8(&mut self, addr: u16, value: u8) {
        log::warn!("attempt made to write {value:#02X} to address {addr:#04X} of ROM-only MBC");
    }

    fn name(&self) -> &str {
        "ROM ONLY"
    }
}
