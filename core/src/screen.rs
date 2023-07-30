use num_derive::FromPrimitive;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Debug, Clone, Copy, FromPrimitive, PartialEq, PartialOrd)]
pub enum Colour {
    White,
    LightGrey,
    DarkGrey,
    Black,
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

    /// Get the colour of the pixel at the given coordinates. If the given coordinates are out of
    /// bounds then black is returned.
    pub fn get(&self, x: u8, y: u8) -> Colour {
        if within_bounds(x, y) {
            self.pixels[index(x, y)]
        } else {
            Colour::Black
        }
    }

    /// Set the colour of the pixel at the given coordinates. If the given coordinates are out of
    /// bounds then nothing happens.
    pub fn set(&mut self, x: u8, y: u8, colour: Colour) {
        if within_bounds(x, y) {
            self.pixels[index(x, y)] = colour;
        }
    }
}

#[inline]
fn within_bounds(x: u8, y: u8) -> bool {
    x < SCREEN_WIDTH as u8 && y < SCREEN_HEIGHT as u8
}

#[inline]
fn index(x: u8, y: u8) -> usize {
    y as usize * SCREEN_WIDTH + x as usize
}
