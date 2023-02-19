const VRAM_SIZE: usize = 0x2000;

pub struct Gpu {
    pub vram: [u8; VRAM_SIZE],
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            vram: [0; VRAM_SIZE],
        }
    }
}
