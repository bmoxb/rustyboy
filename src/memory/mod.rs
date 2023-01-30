// TODO: Proper memory implementation!
pub struct Memory {
    mem: [u8; 0x10000],
}

impl Memory {
    pub fn new() -> Self {
        Memory { mem: [0; 0x10000] }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        log::trace!("at memory address {:#06X}, writing byte {:#04X} (replacing previous value {:#04X})", addr, value, self.read8(addr));
        self.mem[addr as usize] = value;
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let lsb = self.mem[addr as usize] as u16;
        let msb = self.mem[addr as usize + 1] as u16;
        (msb << 8) + lsb
    }

    pub fn write16(&mut self, addr: u16, value: u16) {
        log::trace!("at memory address {:#06X}, writing word {:#06X} (replacing previous value {:#06X})", addr, value, self.read16(addr));
        self.mem[addr as usize] = (value & 0xFF) as u8; // little endian so LSB first
        self.mem[addr as usize + 1] = (value >> 8) as u8;
    }
}
