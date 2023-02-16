use crate::bits::modify_bit;
use crate::cpu::Interrupt;

const INTE_ADDR: u16 = 0xFFFF;
const INTF_ADDR: u16 = 0xFF0F;

// TODO: Proper memory implementation!
pub struct Memory {
    mem: [u8; 0x10000],
    logged_char: Option<char>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            mem: [0; 0x10000],
            logged_char: None,
        }
    }

    pub fn load(&mut self, rom: &[u8]) {
        for (addr, byte) in rom.iter().enumerate() {
            self.mem[addr] = *byte;
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        log::trace!(
            "at memory address {:#06X}, writing byte {:#04X} (replacing previous value {:#04X})",
            addr,
            value,
            self.read8(addr)
        );
        self.mem[addr as usize] = value;

        if addr == 0xFF02 && value == 0x81 {
            self.logged_char = Some(self.read8(0xFF01) as char);
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let lsb = self.mem[addr as usize];
        let msb = self.mem[addr as usize + 1];
        u16::from_be_bytes([msb, lsb])
    }

    pub fn write16(&mut self, addr: u16, value: u16) {
        log::trace!(
            "at memory address {:#06X}, writing word {:#06X} (replacing previous value {:#06X})",
            addr,
            value,
            self.read16(addr)
        );
        self.mem[addr as usize] = (value & 0xFF) as u8; // little endian so LSB first
        self.mem[addr as usize + 1] = (value >> 8) as u8;
    }

    pub fn take_logged_char(&mut self) -> Option<char> {
        self.logged_char.take()
    }

    pub fn interrupt_enable_register(&self) -> u8 {
        self.read8(INTE_ADDR)
    }

    pub fn interrupt_flag_register(&self) -> u8 {
        self.read8(INTF_ADDR)
    }

    pub fn enable_interrupt(&mut self, int: Interrupt, enabled: bool) {
        self.write8(
            INTE_ADDR,
            modify_bit(self.read8(INTE_ADDR), int.bit(), enabled),
        );
    }

    pub fn flag_interrupt(&mut self, int: Interrupt, flagged: bool) {
        self.write8(
            INTF_ADDR,
            modify_bit(self.read8(INTF_ADDR), int.bit(), flagged),
        )
    }
}
