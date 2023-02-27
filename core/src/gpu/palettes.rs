use crate::bits::get_bits;

#[derive(Clone, Copy)]
pub enum Colour {
    White,
    LightGrey,
    DarkGrey,
    Black,
    Transparent,
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
    pub fn colour_for_id(&self, id: u8) -> Colour {
        debug_assert!(id < 4);
        let bits = get_bits(self.0, id * 2, (id + 1) * 2);
        Colour::from(bits)
    }
}

pub struct ObjectPalette(pub u8);

impl ObjectPalette {
    pub fn colour_for_id(&self, id: u8) -> Colour {
        debug_assert!(id < 4);
        if id == 0 {
            Colour::Transparent
        } else {
            let bits = get_bits(self.0, id * 2, (id + 1) * 2);
            Colour::from(bits)
        }
    }
}
