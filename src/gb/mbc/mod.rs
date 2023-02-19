mod rom_only;

pub use rom_only::RomOnly;

use std::path::Path;
use std::{fs, io};

const CARTRIDGE_TYPE: usize = 0x0147;
const TITLE_START: u16 = 0x0134;
const ROM_SIZE: u16 = 0x0148;
const RAM_SIZE: u16 = 0x0149;

pub fn from_rom_file(path: impl AsRef<Path>) -> io::Result<Box<dyn MemoryBankController>> {
    fs::read(path).map(|data| from_rom_data(&data))
}

pub fn from_rom_data(data: &[u8]) -> Box<dyn MemoryBankController> {
    match data[CARTRIDGE_TYPE] {
        0 | 1 => Box::new(RomOnly::new(data)),
        n => unimplemented!("cartridge type {n}"),
    }
}

pub trait MemoryBankController {
    fn read8(&self, addr: u16) -> u8;

    fn write8(&mut self, addr: u16, value: u8);

    fn name(&self) -> &str;

    fn game_title(&self) -> String {
        let mut title = String::new();

        for addr in TITLE_START..TITLE_START + 16 {
            let c = self.read8(addr) as char;
            if c == '\0' {
                break;
            }
            title.push(c);
        }

        title
    }

    fn rom_bank_count(&self) -> u16 {
        let value = self.read8(ROM_SIZE);
        if value > 8 {
            log::warn!("invalid ROM size {value}");
        }
        2_u16.pow(value as u32 + 1)
    }

    fn rom_size(&self) -> usize {
        self.rom_bank_count() as usize * 0x4000
    }

    fn ram_bank_count(&self) -> u16 {
        match self.read8(RAM_SIZE) {
            0 => 0,
            2 => 1,
            3 => 4,
            4 => 16,
            5 => 8,
            value => {
                log::warn!("invalid RAM size {value}");
                0
            }
        }
    }

    fn ram_size(&self) -> usize {
        self.ram_bank_count() as usize * 0x2000
    }
}
