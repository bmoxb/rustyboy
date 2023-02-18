#[derive(Default)]
pub struct InputOutputRegisters {
    pub joypad_input: u8,
    pub serial_transfer_data: u8,
    pub serial_transfer_control: u8,
    pub divider: u8,
    pub timer_counter: u8,
    pub timer_modulo: u8,
    pub timer_control: u8,
    pub interrupt_flag: u8,
    // TODO: Audio and LCD registers.
}

impl InputOutputRegisters {
    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => self.joypad_input,
            0xFF01 => self.serial_transfer_data,
            0xFF02 => self.serial_transfer_control,
            0xFF04 => self.divider,
            0xFF05 => self.timer_counter,
            0xFF06 => self.timer_modulo,
            0xFF07 => self.timer_control,
            0xFF0F => self.interrupt_flag,
            _ => 0,
        }
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        match addr {
            0xFF00 => self.joypad_input = value,
            0xFF01 => self.serial_transfer_data = value,
            0xFF02 => self.serial_transfer_control = value,
            0xFF04 => self.divider = value,
            0xFF05 => self.timer_counter = value,
            0xFF06 => self.timer_modulo = value,
            0xFF07 => self.timer_control = value,
            0xFF0F => self.interrupt_flag = value,
            _ => {}
        };
    }
}
