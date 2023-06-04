use crate::bits::get_bits;

use derive_more::Display;

/// A single 1 byte CPU opcode.
#[derive(Clone, Copy, Display)]
#[display(fmt = "{0:#04X} ({0:#010b})", _0)]
pub struct Opcode(pub u8);

impl Opcode {
    /// Get 3 bits at a 3-bit offset from the opcode.
    pub fn xxx(&self) -> u8 {
        get_bits(self.0, 3, 6)
    }

    /// Get the 3 least significant bits in the opcode.
    pub fn yyy(&self) -> u8 {
        get_bits(self.0, 0, 3)
    }

    /// Get 2 bits at a 4-bit offset from the opcode. These bits are often used to identify a 16-bit register pair
    /// (i.e., one of BC, DE, HL, AF/SP).
    pub fn rr(&self) -> u8 {
        get_bits(self.0, 4, 6)
    }

    /// Get 2 bits at a 3-bit offset from the opcode. These bits are used to identify the particular condition used in a
    /// conditional JP, JR, CALL, or RET instruction.
    pub fn ff(&self) -> u8 {
        get_bits(self.0, 3, 5)
    }
}
