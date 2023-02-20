use crate::bits::get_bit;
use crate::interrupts::{Interrupt, Interrupts};

#[derive(Default, Debug)]
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
        self.divider_cycles += cpu_cycles * 4;

        while self.divider_cycles >= 256 {
            self.divider_cycles -= 256;
            self.divider = self.divider.wrapping_add(1);
        }

        if self.enabled() {
            self.timer_cycles += cpu_cycles * 4;

            while self.timer_cycles >= self.required_cycles() {
                self.increase_counter(interrupts);
                self.timer_cycles -= self.required_cycles();
            }
        }

        log::trace!("timer updated - {self:?}");
    }

    fn enabled(&self) -> bool {
        get_bit(self.control, 2)
    }

    fn required_cycles(&self) -> usize {
        match self.control & 0b11 {
            0 => 1024,
            1 => 16,
            2 => 64,
            3 => 256,
            _ => unreachable!(),
        }
    }

    fn increase_counter(&mut self, interrupts: &mut Interrupts) {
        if self.counter == u8::MAX {
            self.counter = self.modulo;
            interrupts.flag(Interrupt::Timer, true);
        } else {
            self.counter += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bits::modify_bit, interrupts::Interrupts};

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

        t.control = 0xFF; // divider should increase regardless of the control byte

        // ensure divider wraps around to 0
        t.divider = 255;
        t.update(&mut ints, 300);
        assert_eq!(t.divider, 0);
    }

    #[test]
    fn counter() {
        let _ = env_logger::builder().is_test(true).try_init();
        let mut t = Timer {
            control: 0b101, // enable timer, 16 cycles
            ..Default::default()
        };

        let mut ints = Interrupts {
            enable: 0xFF, // enable all
            ..Default::default()
        };

        assert!(!ints.is_flagged(Interrupt::Timer));
        t.update(&mut ints, 2000);
        assert!(ints.is_flagged(Interrupt::Timer));
    }
}
