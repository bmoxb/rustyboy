mod ioregisters;

use ioregisters::InputOutputRegisters;

use crate::bits::modify_bit;
use crate::cpu::Interrupt;
use crate::mbc::MemoryBankController;

const VRAM_SIZE: usize = 0x2000;
const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x7F;

pub struct Memory {
    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    mbc: Box<dyn MemoryBankController>,
    pub io_regs: InputOutputRegisters,
    pub interrupt_enable: u8,
}

impl Memory {
    pub fn new(mbc: Box<dyn MemoryBankController>) -> Self {
        Memory {
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            mbc,
            io_regs: InputOutputRegisters::default(),
            interrupt_enable: 0,
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.mbc.read8(addr),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            0xA000..=0xBFFF => self.mbc.read8(addr),
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            0xE000..=0xFDFF => {
                log::warn!("prohibited address {:#04X} read", addr);
                self.wram[(addr - 0xE000) as usize]
            }
            0xFE00..=0xFE9F => unimplemented!(), // TODO: Sprite attribute table.
            0xFEA0..=0xFEFF => {
                log::warn!("prohibited address {:#04X} read", addr);
                0xFF
            }
            0xFF00..=0xFF7F => self.io_regs.read8(addr),
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupt_enable,
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let lsb = self.read8(addr);
        let msb = self.read8(addr + 1);
        u16::from_be_bytes([msb, lsb])
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        log::trace!(
            "at memory address {:#06X}, writing byte {:#04X} (replacing previous value {:#04X})",
            addr,
            value,
            self.read8(addr)
        );

        match addr {
            0x0000..=0x7FFF => self.mbc.write8(addr, value),
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.mbc.write8(addr, value),
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            0xE000..=0xFDFF => {
                log::warn!("prohibited address {:#04X} read", addr);
                self.wram[(addr - 0xE000) as usize] = value;
            }
            0xFE00..=0xFE9F => unimplemented!(), // TODO: Sprite attribute table.
            0xFEA0..=0xFEFF => log::warn!("prohibited address {:#04X} read", addr),
            0xFF00..=0xFF7F => self.io_regs.write8(addr, value),
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.interrupt_enable = value,
        }
    }

    pub fn write16(&mut self, addr: u16, value: u16) {
        let [msb, lsb] = value.to_be_bytes();
        self.write8(addr, lsb); // little endian so LSB first
        self.write8(addr + 1, msb);
    }

    pub fn flag_interrupt(&mut self, int: Interrupt, value: bool) {
        self.io_regs.interrupt_flag = modify_bit(self.io_regs.interrupt_flag, int.bit(), value);
    }
}
