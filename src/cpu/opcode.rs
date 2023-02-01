use std::fmt;

#[derive(Clone, Copy)]
pub struct Opcode(pub u8);

impl Opcode {
    pub fn xxx(&self) -> u8 {
        (self.0 >> 3) & 0b11
    }

    pub fn yyy(&self) -> u8 {
        self.0 & 0b111
    }

    pub fn reg16(&self) -> u8 {
        (self.0 >> 4) & 0b11
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{0:#04X} ({0:#010b})", self.0)
    }
}
