pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Clone, Copy)]
pub enum Pixel {
    Dark,
    MediumDark,
    MediumLight,
    Light,
}

pub struct Screen {
    buffer: [Pixel; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Screen {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Screen {
            buffer: [Pixel::Dark; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }

    pub fn get(&self, x: u8, y: u8) -> Pixel {
        self.buffer[index(x, y)]
    }

    pub fn set(&mut self, x: u8, y: u8, p: Pixel) {
        self.buffer[index(x, y)] = p;
    }
}

#[inline]
fn index(x: u8, y: u8) -> usize {
    (y as usize * SCREEN_WIDTH) + x as usize
}
