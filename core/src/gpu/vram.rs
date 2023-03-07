pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
const VRAM_SIZE: usize = (VRAM_END - VRAM_START + 1) as usize;

pub struct VideoRam {
    data: [u8; VRAM_SIZE],
}

impl VideoRam {
    pub fn new() -> Self {
        VideoRam {
            data: [0; VRAM_SIZE],
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        self.data[(addr - VRAM_START) as usize]
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        self.data[(addr - VRAM_START) as usize] = value;
    }
}
