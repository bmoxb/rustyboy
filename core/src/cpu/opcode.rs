use std::fmt;

#[derive(Clone, Copy)]
pub struct Opcode(pub u8);

impl Opcode {
    pub fn xxx(&self) -> u8 {
        (self.0 >> 3) & 0b111
    }

    // Get the 3 least significant bits in the opcode.
    pub fn yyy(&self) -> u8 {
        self.0 & 0b111
    }

    // Get the 2 bits at a 4-bit offset from the opcode. These bits are often used to identify a 16-bit register pair
    // (i.e., one of BC, DE, HL, AF/SP).
    pub fn rr(&self) -> u8 {
        (self.0 >> 4) & 0b11
    }

    pub fn ff(&self) -> u8 {
        (self.0 >> 3) & 0b11
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0:#04X} ({0:#010b})", self.0)
    }
}
