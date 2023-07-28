use std::path::Path;
use std::{fmt, fs, io};

const TITLE_START: usize = 0x0134;

pub struct Cartridge {
    data: Vec<u8>,
}

impl Cartridge {
    pub fn from_data(data: Vec<u8>) -> Cartridge {
        Cartridge { data }
    }

    pub fn from_file(path: impl AsRef<Path>) -> io::Result<Cartridge> {
        fs::read(path).map(Cartridge::from_data)
    }

    pub fn read8(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn game_title(&self) -> String {
        self.data[TITLE_START..TITLE_START + 16]
            .iter()
            .map_while(|c| (*c != 0).then_some(*c as char))
            .collect()
    }

    pub fn cart_type(&self) -> CartridgeType {
        match self.data[0x147] {
            0x00 => CartridgeType::RomOnly,
            0x01 => CartridgeType::MBC1 {
                ram: false,
                battery: false,
            },
            0x02 => CartridgeType::MBC1 {
                ram: true,
                battery: false,
            },
            0x03 => CartridgeType::MBC1 {
                ram: true,
                battery: true,
            },
            0x0F => CartridgeType::MBC3 {
                timer: true,
                ram: false,
                battery: true,
            },
            0x10 => CartridgeType::MBC3 {
                timer: true,
                ram: true,
                battery: true,
            },
            0x11 => CartridgeType::MBC3 {
                timer: false,
                ram: false,
                battery: false,
            },
            0x12 => CartridgeType::MBC3 {
                timer: false,
                ram: true,
                battery: false,
            },
            0x13 => CartridgeType::MBC3 {
                timer: false,
                ram: true,
                battery: true,
            },
            n => CartridgeType::Unsupported(n),
        }
    }

    pub fn rom_size(&self) -> usize {
        1 << (self.data[0x148] + 15)
    }

    pub fn ram_size(&self) -> usize {
        match self.data[0x149] {
            2 => 0x2000,  // 8 KiB (1 bank)
            3 => 0x8000,  // 32 KiB (4 banks)
            4 => 0x20000, // 128 KiB (16 banks)
            5 => 0x10000, // 64 KiB (8 banks)
            _ => 0,
        }
    }

    pub fn sold_in_japan(&self) -> bool {
        self.data[0x14A] == 0
    }
}

impl fmt::Display for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} - {}",
            self.game_title(),
            if self.sold_in_japan() {
                "[Japan & overseas]"
            } else {
                "[outside of Japan only]"
            },
            self.cart_type(),
        )
    }
}

#[derive(Clone, Copy)]
pub enum CartridgeType {
    RomOnly,
    MBC1 {
        ram: bool,
        battery: bool,
    },
    MBC3 {
        timer: bool,
        ram: bool,
        battery: bool,
    },
    Unsupported(u8),
}

impl fmt::Display for CartridgeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CartridgeType::RomOnly => write!(f, "ROM only"),
            CartridgeType::MBC1 { ram, battery } => write!(
                f,
                "MBC1{}{}",
                if *ram { "+RAM" } else { "" },
                if *battery { "+BATTERY" } else { "" }
            ),
            CartridgeType::MBC3 {
                timer,
                ram,
                battery,
            } => write!(
                f,
                "MBC3{}{}{}",
                if *timer { "+TIMER" } else { "" },
                if *ram { "+RAM" } else { "" },
                if *battery { "+BATTERY" } else { "" },
            ),
            CartridgeType::Unsupported(n) => write!(f, "unsupported type {:#04x}", n),
        }
    }
}
