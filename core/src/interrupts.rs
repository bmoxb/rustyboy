use std::fmt;

use crate::bits::{get_bit, modify_bit};

#[derive(Default)]
pub struct Interrupts {
    pub enable: u8,
    pub flag: u8,
}

impl Interrupts {
    pub fn flag(&mut self, int: Interrupt, value: bool) {
        self.flag = modify_bit(self.flag, int.bit(), value);
    }

    #[allow(dead_code)]
    pub fn is_flagged(&self, int: Interrupt) -> bool {
        get_bit(self.flag, int.bit())
    }

    pub fn next_triggered_interrupt(&self) -> Option<Interrupt> {
        let triggered = self.flag & self.enable;
        if triggered == 0 {
            return None;
        }

        let index = triggered.trailing_zeros();
        Interrupt::from_index(index)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Interrupt {
    VBlank,
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    fn from_index(index: u32) -> Option<Self> {
        match index {
            0 => Some(Interrupt::VBlank),
            1 => Some(Interrupt::LcdStat),
            2 => Some(Interrupt::Timer),
            3 => Some(Interrupt::Serial),
            4 => Some(Interrupt::Joypad),
            _ => {
                log::warn!("invalid interrupt index {index}");
                None
            }
        }
    }

    pub fn handler_address(&self) -> u16 {
        0x40 + (0x8 * (*self as u16))
    }

    pub fn bit(&self) -> u8 {
        *self as u8
    }
}

impl fmt::Display for Interrupt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} (bit {}, INT 0x{:X})",
            self,
            self.bit(),
            self.handler_address()
        )
    }
}
