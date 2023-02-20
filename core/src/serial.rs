pub struct SerialTransfer {
    pub data: u8,
    pub control: u8,
    byte: Option<u8>,
}

impl SerialTransfer {
    pub fn new() -> Self {
        SerialTransfer {
            data: 0,
            control: 0x7E,
            byte: None,
        }
    }

    pub fn update(&mut self) {
        // transfer requested, use internal clock
        if self.control == 0x81 {
            self.control = 0x01; // no transfer in progress/requested
            self.byte = Some(self.data);
        }
    }

    pub fn take_byte(&mut self) -> Option<u8> {
        self.byte.take()
    }
}
