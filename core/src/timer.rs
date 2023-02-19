use crate::bits::get_bit;
use crate::cpu;
use crate::memory::{InputOutputRegisters, Memory};

#[derive(Default)]
pub struct Timer {
    timer_cycles: usize,
    divider_cycles: usize,
}

impl Timer {
    pub fn update(&mut self, mem: &mut Memory, cpu_cycles: usize) {
        self.divider_cycles += cpu_cycles;

        if self.divider_cycles >= 256 {
            self.divider_cycles -= 256;
            mem.io_regs.divider = mem.io_regs.divider.wrapping_add(1);
        }

        if self.enabled(&mem.io_regs) {
            self.timer_cycles += cpu_cycles;

            let frequency = self.timer_controller_frequency(&mem.io_regs);
            let required_cycles = cpu::CLOCK_SPEED / frequency;

            if self.timer_cycles >= required_cycles {
                self.increase_timer(mem);
                self.timer_cycles -= required_cycles;
            }
        }
    }

    fn enabled(&self, regs: &InputOutputRegisters) -> bool {
        get_bit(regs.timer_control, 2)
    }

    fn timer_controller_frequency(&self, regs: &InputOutputRegisters) -> usize {
        match regs.timer_control & 0b11 {
            0 => 4096,
            1 => 262144,
            2 => 65536,
            3 => 16384,
            _ => unreachable!(),
        }
    }

    fn increase_timer(&self, mem: &mut Memory) {
        if mem.io_regs.timer_counter == u8::MAX {
            mem.io_regs.timer_counter = mem.io_regs.timer_modulo;
            mem.flag_interrupt(cpu::Interrupt::Timer, true);
        } else {
            mem.io_regs.timer_counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mbc;

    #[test]
    fn divider() {
        let mut t = Timer::default();
        let mut mem = Memory::new(Box::new(mbc::RomOnly::new(&[])));

        // increment divider over time
        t.update(&mut mem, 256);
        assert_eq!(mem.io_regs.divider, 1);
        t.update(&mut mem, 300);
        assert_eq!(mem.io_regs.divider, 2);
        t.update(&mut mem, 230);
        assert_eq!(mem.io_regs.divider, 3);

        // ensure divider wraps around to 0
        mem.io_regs.divider = 255;
        t.update(&mut mem, 300);
        assert_eq!(mem.io_regs.divider, 0);

        // ensure divider register is reset when written to
        mem.io_regs.divider = 10;
        mem.write8(0xFF04, 25);
        assert_eq!(mem.io_regs.divider, 0);
    }
}
