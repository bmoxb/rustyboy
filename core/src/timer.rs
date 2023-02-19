use crate::bits::get_bit;
use crate::cpu;
use crate::interrupts::{Interrupt, Interrupts};

#[derive(Default)]
pub struct Timer {
    pub divider: u8,
    pub counter: u8,
    pub modulo: u8,
    pub control: u8,
    timer_cycles: usize,
    divider_cycles: usize,
}

impl Timer {
    pub fn update(&mut self, interrupts: &mut Interrupts, cpu_cycles: usize) {
        self.divider_cycles += cpu_cycles;

        if self.divider_cycles >= 256 {
            self.divider_cycles -= 256;
            self.divider = self.divider.wrapping_add(1);
        }

        if self.enabled() {
            self.timer_cycles += cpu_cycles;

            let frequency = self.timer_controller_frequency();
            let required_cycles = cpu::CLOCK_SPEED / frequency;

            if self.timer_cycles >= required_cycles {
                self.increase_timer(interrupts);
                self.timer_cycles -= required_cycles;
            }
        }
    }

    fn enabled(&self) -> bool {
        get_bit(self.control, 2)
    }

    fn timer_controller_frequency(&self) -> usize {
        match self.control & 0b11 {
            0 => 4096,
            1 => 262144,
            2 => 65536,
            3 => 16384,
            _ => unreachable!(),
        }
    }

    fn increase_timer(&mut self, interrupts: &mut Interrupts) {
        if self.counter == u8::MAX {
            self.counter = self.modulo;
            interrupts.flag_interrupt(Interrupt::Timer, true);
        } else {
            self.counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interrupts::Interrupts;

    #[test]
    fn divider() {
        let mut t = Timer::default();
        let mut ints = Interrupts::default();

        // increment divider over time
        t.update(&mut ints, 256);
        assert_eq!(t.divider, 1);
        t.update(&mut ints, 300);
        assert_eq!(t.divider, 2);
        t.update(&mut ints, 230);
        assert_eq!(t.divider, 3);

        // ensure divider wraps around to 0
        t.divider = 255;
        t.update(&mut ints, 300);
        assert_eq!(t.divider, 0);
    }
}
