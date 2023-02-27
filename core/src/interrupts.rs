use derive_more::Display;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::bits::{get_bit, modify_bit};

pub struct Interrupts {
    pub enable: u8,
    pub flag: u8,
}

impl Interrupts {
    pub fn new() -> Self {
        Interrupts {
            enable: 0,
            flag: 0xE1,
        }
    }
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
        Interrupt::from_u32(index)
    }
}

#[derive(Debug, Display, Copy, Clone, FromPrimitive)]
#[display(fmt = "{} interrupt")]
pub enum Interrupt {
    VBlank,
    #[display(fmt = "LCD STAT")]
    LcdStat,
    Timer,
    Serial,
    Joypad,
}

impl Interrupt {
    pub fn handler_address(&self) -> u16 {
        0x40 + (0x8 * (*self as u16))
    }

    pub fn bit(&self) -> u8 {
        *self as u8
    }
}
