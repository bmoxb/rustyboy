#[derive(Clone, Copy)]
pub struct Opcode {
    pub value: u8,
}

impl Opcode {
    pub fn dst8(&self) -> u8 {
        (self.value >> 3) & 0b11
    }

    pub fn src8(&self) -> u8 {
        self.value & 0b111
    }

    pub fn reg16(&self) -> u8 {
        (self.value >> 4) & 0b11
    }
}
