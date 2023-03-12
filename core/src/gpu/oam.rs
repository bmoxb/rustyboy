use crate::bits::get_bit;

pub const OAM_START: u16 = 0xFE00;
pub const OAM_END: u16 = 0xFE9F;
const OAM_SIZE: usize = (OAM_END - OAM_START + 1) as usize;

pub const SPRITE_COUNT: u16 = 40;

pub struct SpriteAttributeTable {
    data: [u8; OAM_SIZE],
}

impl SpriteAttributeTable {
    pub fn new() -> Self {
        SpriteAttributeTable {
            data: [0; OAM_SIZE],
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        debug_assert!((OAM_START..=OAM_END).contains(&addr));
        self.data[(addr - OAM_START) as usize]
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        debug_assert!((OAM_START..=OAM_END).contains(&addr));
        self.data[(addr - OAM_START) as usize] = value;
    }

    pub fn read_sprite(&self, index: u16) -> Sprite {
        debug_assert!(index < SPRITE_COUNT);
        let addr = OAM_START + (index * 4);
        let attributes = self.read8(addr + 3);
        Sprite {
            y: self.read8(addr),
            x: self.read8(addr + 1),
            tile_index: self.read8(addr + 2),
            bg_and_window_over_sprite: get_bit(attributes, 7),
            y_flip: get_bit(attributes, 6),
            x_flip: get_bit(attributes, 5),
            use_palette_1: get_bit(attributes, 4),
        }
    }
}

pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile_index: u8,
    pub bg_and_window_over_sprite: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub use_palette_1: bool,
}
