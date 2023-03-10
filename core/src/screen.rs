use num_derive::FromPrimitive;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Debug, Clone, Copy, FromPrimitive)]
pub enum Colour {
    White,
    LightGrey,
    DarkGrey,
    Black,
    Transparent,
}

pub struct Screen {
    pixels: [Colour; SCREEN_WIDTH * SCREEN_HEIGHT],
}

impl Screen {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Screen {
            pixels: [Colour::DarkGrey; SCREEN_WIDTH * SCREEN_HEIGHT],
        }
    }
    pub fn get(&self, x: u8, y: u8) -> Colour {
        self.pixels[index(x, y)]
    }

    pub fn set(&mut self, x: u8, y: u8, colour: Colour) {
        self.pixels[index(x, y)] = colour
    }
}

#[inline]
fn index(x: u8, y: u8) -> usize {
    y as usize * SCREEN_WIDTH + x as usize
}
