use std::cmp;

pub struct MBC1 {
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_bank: u8,
    ram_bank: u8,
    ram_enable: bool,
    expansion_mode: bool,
}

impl MBC1 {
    pub fn new(data: &[u8]) -> Self {
        let ram_banks = data[super::RAM_SIZE as usize];
        let ram_size = ram_banks as usize * 0x2000;

        MBC1 {
            rom: data.to_vec(),
            ram: vec![0; ram_size],
            rom_bank: 1,
            ram_bank: 0,
            ram_enable: false,
            expansion_mode: false,
        }
    }
}

impl super::MemoryBankController for MBC1 {
    fn mbc_name(&self) -> &str {
        "MBC1"
    }

    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.rom[addr as usize],
            0x4000..=0x7FFF => self.rom[(self.rom_bank as usize * 0x4000) + addr as usize],
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    self.ram[(self.ram_bank as usize * 0x2000) + addr as usize]
                } else {
                    0xFF
                }
            }
            _ => 0,
        }
    }

    fn write8(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enable = (value & 0x0F) == 0x0A,
            0x2000..=0x3FFF => {
                // the two high bits of the ROM bank number are set in the 0x400..=0x5FFF range so we need to avoid
                // altering them here
                let two_high_bits = self.rom_bank & 0x60;

                self.rom_bank = two_high_bits + cmp::max(1, value & 0x1F); // discard the 3 most significant bits
            }
            0x4000..=0x5FFF => {
                if self.expansion_mode {
                    self.ram_bank = value & 0b11;
                } else {
                    let five_low_bits = self.rom_bank & 0x1F; // TODO: use get_bits
                    self.rom_bank = five_low_bits + ((value & 0b11) << 5);
                }
            }
            0x6000..=0x7FFF => self.expansion_mode = value != 0,
            0xA000..=0xBFFF => {
                if self.ram_enable {
                    self.ram[(self.ram_bank as usize * 0x2000) + addr as usize] = value;
                }
            }
            _ => {}
        }
    }
}
