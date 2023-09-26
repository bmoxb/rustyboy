#[cfg(test)]
mod tests;

mod bits;
pub mod cartridge;
pub mod cpu;
mod gpu;
mod interrupts;
pub mod joypad;
pub mod mbc;
pub mod memory;
pub mod screen;
mod serial;
mod timer;

use cpu::Cpu;
use joypad::Joypad;
use mbc::MemoryBankController;
use memory::Memory;
use screen::Screen;

/// Type to represent some number of cycles. Note that this emulator exclusively uses T-Cycles
/// rather than M-Cycles or any mixing of two.
/// 1 M-Cycle = 4 T-Cycles
type Cycles = u32;

const CYCLES_PER_SECOND: Cycles = 4194304;

/// Game Boy console emulator.
pub struct GameBoy {
    pub cpu: Cpu,
    pub mem: Memory,
}

impl GameBoy {
    pub fn new(mbc: Box<dyn MemoryBankController>) -> Self {
        GameBoy {
            cpu: Cpu::new(),
            mem: Memory::new(mbc),
        }
    }

    /// Update the state of the console - fetch and execute CPU instructions, handle interrupts, update the timer,
    /// handle rendering, etc. The `delta` parameter must express in seconds how long has passed since the last update.
    pub fn update(&mut self, delta: f32) {
        let total_cycles_this_update = (delta * CYCLES_PER_SECOND as f32) as Cycles;
        let mut cycles_so_far = 0;

        while cycles_so_far < total_cycles_this_update {
            let cycles = self.step();
            cycles_so_far += cycles;
        }
    }

    /// Perform a single update 'step'. In other words, fetch and execute a single CPU instruction and based on the
    /// number of cycles required by that instruction, update the other components of the system.
    pub fn step(&mut self) -> Cycles {
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

    pub fn take_serial_byte(&mut self) -> Option<u8> {
        self.mem.serial.take_byte()
    }
}
