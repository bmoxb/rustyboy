const VRAM_SIZE: usize = 0x2000;

pub struct Gpu {
    pub vram: [u8; VRAM_SIZE],
    pub lcd_control: u8,
    pub lcd_status: u8,
    pub viewport_y: u8,
    pub viewport_x: u8,
    pub lcd_y: u8,
    pub ly_compare: u8,
    pub bg_palette_data: u8,
    pub obj_palette_0_data: u8,
    pub obj_palette_1_data: u8,
    pub window_y: u8,
    pub window_x: u8,
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            vram: [0; VRAM_SIZE],
            lcd_control: 0x91,
            lcd_status: 0x81,
            viewport_y: 0,
            viewport_x: 0,
            lcd_y: 0x91,
            ly_compare: 0,
            bg_palette_data: 0xFC,
            obj_palette_0_data: 0,
            obj_palette_1_data: 0,
            window_y: 0,
            window_x: 0,
        }
    }
}
