use crate::bits::{bit_accessors, get_bits};
use crate::register_type;

const VRAM_SIZE: usize = 0x2000;

pub struct Gpu {
    pub vram: [u8; VRAM_SIZE],
    pub lcd_control: LcdControlRegister,
    pub lcd_status: LcdStatusRegister,
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
            lcd_control: LcdControlRegister(0x91),
            lcd_status: LcdStatusRegister(0x81),
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

register_type!(LcdControlRegister);

#[allow(dead_code)]
impl LcdControlRegister {
    bit_accessors!(0, bg_and_window_enable);
    bit_accessors!(1, obj_enable);
    bit_accessors!(2, obj_size);
    bit_accessors!(3, bg_tile_map_area);
    bit_accessors!(4, bg_and_window_tile_data_area);
    bit_accessors!(5, window_enable);
    bit_accessors!(6, window_tile_map_area);
    bit_accessors!(7, lcd_enable);
}

register_type!(LcdStatusRegister);

#[allow(dead_code)]
impl LcdStatusRegister {
    fn status(&self) -> LcdStatus {
        match get_bits(self.0, 0, 2) {
            0 => LcdStatus::HBlank,
            1 => LcdStatus::VBlank,
            2 => LcdStatus::SearchingOAM,
            3 => LcdStatus::TransferringDataToController,
            _ => unreachable!(),
        }
    }
    bit_accessors!(2, ly_and_lyc_are_equal);
}

enum LcdStatus {
    HBlank,
    VBlank,
    SearchingOAM,
    TransferringDataToController,
}
