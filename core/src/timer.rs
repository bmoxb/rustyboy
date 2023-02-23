use crate::bits::{get_bit, get_bits};
use crate::interrupts::{Interrupt, Interrupts};

#[derive(Debug)]
pub struct Timer {
    pub divider: u8,
    pub counter: u8,
    pub modulo: u8,
    pub control: u8,
    timer_cycles: usize,
    divider_cycles: usize,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            divider: 0x18,
            counter: 0,
            modulo: 0,
            control: 0xF8,
            timer_cycles: 0,
            divider_cycles: 0,
        }
    }

    pub fn update(&mut self, interrupts: &mut Interrupts, cpu_cycles: usize) {
        self.divider_cycles += cpu_cycles;

        while self.divider_cycles >= 64 {
            self.divider_cycles -= 64;
            self.divider = self.divider.wrapping_add(1);
        }

        if self.enabled() {
            self.timer_cycles += cpu_cycles;

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
        match get_bits(self.control, 0, 2) {
            0 => 256,
            1 => 4,
            2 => 16,
            3 => 64,
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
        let mut t = Timer::new();
        t.divider = 0;

        let mut ints = Interrupts::new();

        // increment divider over time
        t.update(&mut ints, 64);
        assert_eq!(t.divider, 1);
        t.update(&mut ints, 75);
        assert_eq!(t.divider, 2);
        t.update(&mut ints, 60);
        assert_eq!(t.divider, 3);

        t.control = 0xFF; // divider should increase regardless of the control byte

        // ensure divider wraps around to 0
        t.divider = 255;
        t.update(&mut ints, 80);
        assert_eq!(t.divider, 0);
    }

    #[test]
    fn counter() {
        let mut t = Timer::new();
        t.control = 0b101; // enable timer, 4 cycles

        let mut ints = Interrupts::new();
        ints.enable = 0xFF; // enable all

        // ensure interrupt occurs and counter wraps around to modulo

        t.update(&mut ints, 1000);
        assert!(!ints.is_flagged(Interrupt::Timer));

        t.modulo = 50;

        t.update(&mut ints, 25);
        assert!(ints.is_flagged(Interrupt::Timer));
        assert_eq!(t.counter, t.modulo);

        // ensure nothing happens when timer is disabled

        ints.flag(Interrupt::Timer, false);

        t.control = modify_bit(t.control, 2, false);
        assert!(!t.enabled());

        t.update(&mut ints, 2000);
        assert_eq!(t.counter, t.modulo); // counter unchanged
        assert!(!ints.is_flagged(Interrupt::Timer)); // no interrupt flagged
    }
}
