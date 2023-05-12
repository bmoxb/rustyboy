#[cfg(test)]
mod tests;

mod bits;
pub mod cartridge;
mod cpu;
mod cycles;
mod gpu;
mod interrupts;
pub mod joypad;
pub mod mbc;
mod memory;
pub mod screen;
mod serial;
mod timer;

use std::io::Write;

use cpu::Cpu;
use cycles::MCycles;
use joypad::Joypad;
use mbc::MemoryBankController;
use memory::Memory;
use screen::Screen;

const CYCLES_PER_SECOND: MCycles = MCycles(1048576);

pub struct GameBoy {
    cpu: Cpu,
    mem: Memory,
    gb_doctor_logging: Option<Box<dyn Write>>,
}

impl GameBoy {
    pub fn new(mbc: Box<dyn MemoryBankController>) -> Self {
        GameBoy {
            cpu: Cpu::new(),
            mem: Memory::new(mbc),
            gb_doctor_logging: None,
        }
    }

    pub fn update(&mut self, delta: f32) {
        let total_cycles_this_update = (delta * CYCLES_PER_SECOND.0 as f32) as u32;
        let mut cycles_so_far = 0;

        while cycles_so_far < total_cycles_this_update {
            let cycles = self.step();
            cycles_so_far += cycles.0;
        }
    }

    pub fn step(&mut self) -> MCycles {
        if let Some(dst) = &mut self.gb_doctor_logging {
            writeln!(
                *dst,
                "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
                self.cpu.regs.a,
                self.cpu.regs.flags.0,
                self.cpu.regs.b,
                self.cpu.regs.c,
                self.cpu.regs.d,
                self.cpu.regs.e,
                self.cpu.regs.h,
                self.cpu.regs.l,
                self.cpu.regs.sp,
                self.cpu.regs.pc,
                self.mem.read8(self.cpu.regs.pc),
                self.mem.read8(self.cpu.regs.pc+1),
                self.mem.read8(self.cpu.regs.pc+2),
                self.mem.read8(self.cpu.regs.pc+3),
            ).unwrap();
        }

        let cycles = self.cpu.cycle(&mut self.mem);
        self.mem.update(cycles);
        cycles
    }

    pub fn joypad(&mut self) -> &mut Joypad {
        &mut self.mem.joypad
    }

    pub fn screen(&self) -> &Screen {
        &self.mem.gpu.screen
    }

    pub fn enable_gb_doctor_logging(&mut self, destination: Box<dyn Write>) {
        self.gb_doctor_logging = Some(destination)
    }

    pub fn take_serial_byte(&mut self) -> Option<u8> {
        self.mem.serial.take_byte()
    }
}
