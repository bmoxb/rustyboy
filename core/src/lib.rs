#[cfg(test)]
mod tests;

mod bits;
mod cpu;
mod gpu;
mod interrupts;
mod joypad;
pub mod mbc;
mod memory;
mod serial;
mod timer;

use cpu::Cpu;
use joypad::Joypad;
use mbc::MemoryBankController;
use memory::Memory;

pub struct GameBoy {
    cpu: Cpu,
    mem: Memory,
    pub gb_doctor_logging: Option<Box<dyn std::io::Write>>,
}

impl GameBoy {
    pub fn new(mbc: Box<dyn MemoryBankController>) -> Self {
        GameBoy {
            cpu: Cpu::new(),
            mem: Memory::new(mbc),
            gb_doctor_logging: None,
        }
    }

    pub fn update(&mut self, _delta: f32) {
        // TODO: Proper timing.

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

        let cpu_cycles = self.cpu.cycle(&mut self.mem);
        self.mem.update(cpu_cycles);
    }

    pub fn joypad(&mut self) -> &Joypad {
        &mut self.mem.joypad
    }

    pub fn take_serial_byte(&mut self) -> Option<u8> {
        self.mem.serial.take_byte()
    }
}

macro_rules! register_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
        pub struct $name(pub u8);
    };
}
pub(crate) use register_type;
