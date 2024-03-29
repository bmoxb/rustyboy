use crate::gpu::oam::{OAM_END, OAM_SIZE, OAM_START};
use crate::gpu::vram::{VRAM_END, VRAM_START};
use crate::gpu::Gpu;
use crate::interrupts::Interrupts;
use crate::joypad::Joypad;
use crate::mbc::MemoryBankController;
use crate::serial::SerialTransfer;
use crate::timer::Timer;
use crate::Cycles;

const WRAM_START: u16 = 0xC000;
const WRAM_END: u16 = 0xDFFF;
const WRAM_SIZE: usize = (WRAM_END - WRAM_START + 1) as usize;

const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

const HRAM_START: u16 = 0xFF80;
const HRAM_END: u16 = 0xFFFE;
const HRAM_SIZE: usize = (HRAM_END - HRAM_START + 1) as usize;

const OAM_TRANSFER_PERIOD: Cycles = 640;

/// Represents both general-purpose RAM (working RAM and high RAM) as well as manages certain components of the full
/// system that are interacted with via the memory bus (the GPU, timer, interrupt system, serial, joypad, and OAM
/// transfer).
pub struct MemoryBus {
    mbc: Box<dyn MemoryBankController>,
    pub gpu: Gpu,
    timer: Timer,
    pub interrupts: Interrupts,
    pub serial: SerialTransfer,
    pub joypad: Joypad,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    oam_transfer_source: u8,
    pending_oam_transfer: bool,
    oam_transfer_clock: Cycles,
}

impl MemoryBus {
    pub fn new(mbc: Box<dyn MemoryBankController>) -> Self {
        MemoryBus {
            mbc,
            gpu: Gpu::new(),
            timer: Timer::new(),
            interrupts: Interrupts::new(),
            serial: SerialTransfer::new(),
            joypad: Joypad::new(),
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            oam_transfer_source: 0,
            pending_oam_transfer: false,
            oam_transfer_clock: 0,
        }
    }

    /// Update the components of the emulator interacted with via the memory bus given that a specified number of CPU
    /// cycles have elapsed.
    pub fn update(&mut self, cycles: Cycles) {
        self.gpu.update(&mut self.interrupts, cycles);
        self.timer.update(&mut self.interrupts, cycles);
        self.serial.update();
        self.update_oam_transfer(cycles);
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7FFF => self.mbc.read8(addr),
            VRAM_START..=VRAM_END => self.gpu.vram.read8(addr),
            0xA000..=0xBFFF => self.mbc.read8(addr),
            WRAM_START..=WRAM_END => self.wram[(addr - WRAM_START) as usize],
            ECHO_RAM_START..=ECHO_RAM_END => {
                log::warn!("prohibited address {:#04X} read (ECHO RAM)", addr);
                self.wram[(addr - ECHO_RAM_START) as usize]
            }
            OAM_START..=OAM_END => self.gpu.oam.read8(addr),
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
            0xFF46 => self.oam_transfer_source,
            0xFF47 => self.gpu.bg_palette_data.0,
            0xFF48 => self.gpu.obj_palette_0_data.0,
            0xFF49 => self.gpu.obj_palette_1_data.0,
            0xFF4A => self.gpu.window_y,
            0xFF4B => self.gpu.window_x_plus_7,
            HRAM_START..=HRAM_END => self.hram[(addr - HRAM_START) as usize],
            0xFFFF => self.interrupts.enable,
            _ => {
                log::trace!("{:#04X}", addr);
                0
            }
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
            VRAM_START..=VRAM_END => self.gpu.vram.write8(addr, value),
            0xA000..=0xBFFF => self.mbc.write8(addr, value),
            WRAM_START..=WRAM_END => self.wram[(addr - WRAM_START) as usize] = value,
            ECHO_RAM_START..=ECHO_RAM_END => {
                log::warn!("prohibited address {:#04X} written to (ECHO RAM)", addr);
                self.wram[(addr - ECHO_RAM_START) as usize] = value;
            }
            OAM_START..=OAM_END => self.gpu.oam.write8(addr, value),
            0xFEA0..=0xFEFF => log::warn!("prohibited address {:#04X} written to", addr),
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
            0xFF44 => {} // LCD Y is read-only
            0xFF45 => self.gpu.ly_compare = value,
            0xFF46 => {
                self.oam_transfer_source = value;
                self.pending_oam_transfer = true;
                self.oam_transfer_clock = 0;
            }
            0xFF47 => self.gpu.bg_palette_data.0 = value,
            0xFF48 => self.gpu.obj_palette_0_data.0 = value,
            0xFF49 => self.gpu.obj_palette_1_data.0 = value,
            0xFF4A => self.gpu.window_y = value,
            0xFF4B => self.gpu.window_x_plus_7 = value,
            HRAM_START..=HRAM_END => self.hram[(addr - HRAM_START) as usize] = value,
            0xFFFF => self.interrupts.enable = value,
            _ => log::trace!("{:#04X}", addr),
        }
    }

    pub fn write16(&mut self, addr: u16, value: u16) {
        let [msb, lsb] = value.to_be_bytes();
        self.write8(addr, lsb); // little endian so LSB first
        self.write8(addr + 1, msb);
    }

    fn update_oam_transfer(&mut self, cycles: Cycles) {
        if self.pending_oam_transfer {
            self.oam_transfer_clock += cycles;

            if self.oam_transfer_clock >= OAM_TRANSFER_PERIOD {
                self.perform_oam_transfer();
                self.pending_oam_transfer = false;
            }
        }
    }

    fn perform_oam_transfer(&mut self) {
        let start_address = self.oam_transfer_source as u16 * 0x100;

        for (offset, addr) in (start_address..(start_address + OAM_SIZE as u16)).enumerate() {
            let value = self.read8(addr);
            self.gpu.oam.write8(OAM_START + offset as u16, value);
        }
    }
}
