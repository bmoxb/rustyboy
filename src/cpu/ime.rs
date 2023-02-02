#[derive(Default)]
pub struct InterruptMasterEnable {
    value: bool,
    enable_in_cycles: u8,
    disable_in_cycles: u8,
}

impl InterruptMasterEnable {
    pub fn enabled(&self) -> bool {
        self.value
    }

    pub fn cycle(&mut self) {
        if self.enable_in_cycles == 0 {
            self.value = true;
            log::trace!("interrupts enabled (IME=1)");
        } else {
            self.enable_in_cycles -= 1;
        }

        if self.disable_in_cycles == 0 {
            self.value = false;
            log::trace!("interrupts disabled (IME=0)");
        } else {
            self.disable_in_cycles -= 1;
        }
    }

    pub fn enable(&mut self, after_cycles: u8) {
        self.enable_in_cycles = after_cycles;
    }

    pub fn disable(&mut self, after_cycles: u8) {
        self.disable_in_cycles = after_cycles;
    }
}
