use num_traits::FromPrimitive;

use crate::bits::get_bits;
use crate::screen::Colour;

pub struct BackgroundPalette(pub u8);

impl BackgroundPalette {
    pub fn colour_for_id(&self, id: u8) -> Colour {
        debug_assert!(id < 4);
        let bits = get_bits(self.0, id * 2, (id + 1) * 2);
        Colour::from_u8(bits).unwrap()
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
            Colour::from_u8(bits).unwrap()
        }
    }
}
