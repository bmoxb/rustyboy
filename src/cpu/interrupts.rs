use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn from_index(index: u32) -> Option<Self> {
        match index {
            0 => Some(Interrupt::VBlank),
            1 => Some(Interrupt::LcdStat),
            2 => Some(Interrupt::Timer),
            3 => Some(Interrupt::Serial),
            4 => Some(Interrupt::Joypad),
            _ => None,
        }
    }
    pub fn handler_address(&self) -> u16 {
        0x40 + (0x8 * (*self as u16))
    }

    pub fn mask(&self) -> u8 {
        1 << (*self as u8)
    }
}

impl fmt::Display for Interrupt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} (bit {}, INT 0x{:X})",
            self,
            *self as u8,
            self.handler_address()
        )
    }
}
