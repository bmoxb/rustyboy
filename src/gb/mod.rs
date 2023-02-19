#[cfg(test)]
mod tests;

mod cpu;
pub mod mbc;
mod memory;
mod timer;

use cpu::Cpu;
use mbc::MemoryBankController;
use memory::Memory;
use timer::Timer;

pub struct GameBoy {
    cpu: Cpu,
    mem: Memory,
    timer: Timer,
}

impl GameBoy {
    pub fn new(mbc: Box<dyn MemoryBankController>) -> Self {
        GameBoy {
            cpu: Cpu::new(),
            mem: Memory::new(mbc),
            timer: Timer::default(),
        }
    }

    pub fn update(&mut self, _delta: f32) {
        // TODO: Proper timing.

        let cpu_cycles = self.cpu.cycle(&mut self.mem);
        self.timer.update(&mut self.mem, cpu_cycles);
    }
}
