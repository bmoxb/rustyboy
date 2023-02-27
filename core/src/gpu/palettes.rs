use crate::bits::get_bits;

#[derive(Clone, Copy)]
pub enum Colour {
    White,
    LightGrey,
    DarkGrey,
    Black,
}

impl From<u8> for Colour {
    fn from(value: u8) -> Self {
        match value {
            0 => Colour::White,
            1 => Colour::LightGrey,
            2 => Colour::DarkGrey,
            3 => Colour::Black,
            _ => panic!("invalid colour value {value}"),
        }
    }
}

pub struct BackgroundPalette(pub u8);

impl BackgroundPalette {
    pub fn colour_for_index(&self, i: u8) -> Colour {
        debug_assert!(i < 4);
        Colour::from(get_bits(self.0, i * 2, (i + 1) * 2))
    }
}
