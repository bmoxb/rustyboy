use std::fmt;

/// The Interrupt Master Enable (IME) is a memory register that is required to be set for interrupts to be handled (see
/// [`crate::cpu::Cpu::handle_interrupts`]).
pub struct InterruptMasterEnable {
    value: bool,
    enable_in_cycles: Option<u8>,
    disable_in_cycles: Option<u8>,
}

impl InterruptMasterEnable {
    pub fn new(value: bool) -> Self {
        InterruptMasterEnable {
            value,
            enable_in_cycles: None,
            disable_in_cycles: None,
        }
    }

    pub fn enabled(&self) -> bool {
        self.value
    }

    /// This method should be called once every CPU cycle so as to handle any delayed updates to the IME register.
    pub fn cycle(&mut self) {
        if matches!(self.enable_in_cycles, Some(0)) {
            self.value = true;
            self.enable_in_cycles.take();
            log::trace!("interrupts enabled (IME=1)");
        } else {
            self.enable_in_cycles = self.enable_in_cycles.map(|x| x - 1);
        }

        if matches!(self.disable_in_cycles, Some(0)) {
            self.value = false;
            self.disable_in_cycles.take();
            log::trace!("interrupts disabled (IME=0)");
        } else {
            self.disable_in_cycles = self.disable_in_cycles.map(|x| x - 1);
        }
    }

    /// Enable the IME register after the specified number of CPU cycles have passed. Enabling IME can be delayed as the
    /// EI instruction for example sets IME with a one cycle delay.
    pub fn enable(&mut self, after_cycles: u8) {
        self.enable_in_cycles = Some(after_cycles);
    }

    /// Disable the IME register after the specified number of CPU cycles have passed.
    pub fn disable(&mut self, after_cycles: u8) {
        self.disable_in_cycles = Some(after_cycles);
    }
}

impl fmt::Display for InterruptMasterEnable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IME={}", self.value as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::InterruptMasterEnable;

    #[test]
    fn enable_disable_immediately() {
        let mut ime = InterruptMasterEnable::new(false);

        ime.enable(0);
        assert!(!ime.enabled());
        ime.cycle();
        assert!(ime.enabled());

        ime.disable(0);
        assert!(ime.enabled());
        ime.cycle();
        assert!(!ime.enabled());
    }

    #[test]
    fn enable_disable_after_cycles() {
        let mut ime = InterruptMasterEnable::new(false);

        ime.enable(1);
        for _ in 0..2 {
            assert!(!ime.enabled());
            ime.cycle();
        }
        assert!(ime.enabled());

        ime.disable(3);
        for _ in 0..4 {
            assert!(ime.enabled());
            ime.cycle();
        }
        assert!(!ime.enabled());
    }
}
