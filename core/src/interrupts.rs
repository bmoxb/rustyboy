use derive_more::Display;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::bits::{get_bit, modify_bit};

/// Represents the interrupt enable and flag registers.
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

    /// Flag an interrupt so that it will be handled (the CPU will call the relevant interrupt handler when an interrupt
    /// is both flagged, enabled, and IME is set).
    pub fn flag(&mut self, int: Interrupt, value: bool) {
        self.flag = modify_bit(self.flag, int.bit(), value);
    }

    #[allow(dead_code)]
    pub fn is_flagged(&self, int: Interrupt) -> bool {
        get_bit(self.flag, int.bit())
    }

    /// Returns the highest-priority interrupt that is both flagged and enabled (if any). The priority order of
    /// interrupts (from highest to lowest) is: VBlank, LCD STAT, timer, serial, joypad.
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
    /// Calculates the address in memory of the interrupt handler for this interrupt.
    pub fn handler_address(&self) -> u16 {
        0x40 + (0x8 * (*self as u16))
    }

    /// Get the number of the bit for this interrupt in the interrupt enable and flag bytes.
    pub fn bit(&self) -> u8 {
        *self as u8
    }
}
