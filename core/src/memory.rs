use crate::gpu::Gpu;
use crate::interrupts::Interrupts;
use crate::joypad::Joypad;
use crate::mbc::MemoryBankController;
use crate::serial::SerialTransfer;
use crate::timer::Timer;
use crate::Display;

const WRAM_SIZE: usize = 0x2000;
const HRAM_SIZE: usize = 0x7F;

pub struct Memory {
    mbc: Box<dyn MemoryBankController>,
    gpu: Gpu,
    timer: Timer,
    pub interrupts: Interrupts,
    pub serial: SerialTransfer,
    pub joypad: Joypad,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
}

impl Memory {
    pub fn new(mbc: Box<dyn MemoryBankController>, display: Box<dyn Display>) -> Self {
        Memory {
            mbc,
            gpu: Gpu::new(display),
            timer: Timer::new(),
            interrupts: Interrupts::new(),
            serial: SerialTransfer::new(),
            joypad: Joypad::new(),
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
        }
    }

    pub fn update(&mut self, cpu_cycles: usize) {
        let t_cycles = cpu_cycles * 4;

        self.timer.update(&mut self.interrupts, cpu_cycles);
        self.serial.update();
        self.gpu.update(t_cycles);
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.mbc.read8(addr),
            0x8000..=0x9FFF => self.gpu.vram[(addr - 0x8000) as usize],
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
            0xFF00 => self.joypad.get_byte(),
            0xFF01 => self.serial.data,
            0xFF02 => self.serial.control,
            0xFF04 => self.timer.divider,
            0xFF05 => self.timer.counter,
            0xFF06 => self.timer.modulo,
            0xFF07 => self.timer.control,
            0xFF0F => self.interrupts.flag,
            0xFF10..=0xFF3F => 0, // TODO: audio
            0xFF40 => self.gpu.lcd_control.0,
            0xFF41 => self.gpu.lcd_status.0,
            0xFF42 => self.gpu.viewport_y,
            0xFF43 => self.gpu.viewport_x,
            0xFF44 => self.gpu.lcd_y,
            0xFF45 => self.gpu.ly_compare,
            0xFF46 => 0, // TODO
            0xFF47 => self.gpu.bg_palette_data,
            0xFF48 => self.gpu.obj_palette_0_data,
            0xFF49 => self.gpu.obj_palette_1_data,
            0xFF4A => self.gpu.window_y,
            0xFF4B => self.gpu.window_x,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            0xFFFF => self.interrupts.enable,
            _ => unimplemented!(),
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
            0x8000..=0x9FFF => self.gpu.vram[(addr - 0x8000) as usize] = value,
            0xA000..=0xBFFF => self.mbc.write8(addr, value),
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            0xE000..=0xFDFF => {
                log::warn!("prohibited address {:#04X} read", addr);
                self.wram[(addr - 0xE000) as usize] = value;
            }
            0xFE00..=0xFE9F => unimplemented!(), // TODO: Sprite attribute table.
            0xFEA0..=0xFEFF => log::warn!("prohibited address {:#04X} read", addr),
            0xFF00 => self.joypad.set_byte(value),
            0xFF01 => self.serial.data = value,
            0xFF02 => self.serial.control = value,
            0xFF04 => self.timer.divider = value,
            0xFF05 => self.timer.counter = value,
            0xFF06 => self.timer.modulo = value,
            0xFF07 => self.timer.control = value,
            0xFF0F => self.interrupts.flag = value,
            0xFF10..=0xFF3F => {} // TODO: audio
            0xFF40 => self.gpu.lcd_control.0 = value,
            0xFF41 => self.gpu.lcd_status.0 = value,
            0xFF42 => self.gpu.viewport_y = value,
            0xFF43 => self.gpu.viewport_x = value,
            0xFF44 => {}
            0xFF45 => self.gpu.ly_compare = value,
            0xFF46 => {} // TODO
            0xFF47 => self.gpu.bg_palette_data = value,
            0xFF48 => self.gpu.obj_palette_0_data = value,
            0xFF49 => self.gpu.obj_palette_1_data = value,
            0xFF4A => self.gpu.window_y = value,
            0xFF4B => self.gpu.window_x = value,
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            0xFFFF => self.interrupts.enable = value,
            _ => unimplemented!(),
        }
    }

    pub fn write16(&mut self, addr: u16, value: u16) {
        let [msb, lsb] = value.to_be_bytes();
        self.write8(addr, lsb); // little endian so LSB first
        self.write8(addr + 1, msb);
    }
}
