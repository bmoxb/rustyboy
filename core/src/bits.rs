pub fn modify_bit(value: u8, bit: u8, set_to: bool) -> u8 {
    let mask = 1 << bit;
    if set_to {
        value | mask
    } else {
        value & !mask
    }
}

pub fn toggle_bit(value: u8, bit: u8) -> u8 {
    let mask = 1 << bit;
    value ^ mask
}

pub fn get_bit(value: u8, bit: u8) -> bool {
    let mask = 1 << bit;
    (value & mask) != 0
}

pub fn get_bits(value: u8, from_inclusive: u8, to_exclusive: u8) -> u8 {
    debug_assert!(from_inclusive <= to_exclusive);
    let mask = 2_u8.pow((to_exclusive - from_inclusive) as u32) - 1;
    (value >> from_inclusive) & mask
}

macro_rules! bit_accessors {
    ($bit:literal, $get:ident) => {
        pub fn $get(&self) -> bool {
            crate::bits::get_bit(self.0, $bit)
        }
    };

    ($bit:literal, $get:ident, $set:ident) => {
        bit_accessors!($bit, $get);

        pub fn $set(&mut self, value: bool) {
            self.0 = crate::bits::modify_bit(self.0, $bit, value);
        }
    };

    ($bit:literal, $get:ident, $set:ident, $toggle:ident) => {
        bit_accessors!($bit, $get, $set);

        pub fn $toggle(&mut self) {
            self.0 = crate::bits::toggle_bit(self.0, $bit);
        }
    };
}
pub(crate) use bit_accessors;

#[cfg(test)]
mod tests {
    #[test]
    fn get_bits() {
        assert_eq!(super::get_bits(0b11100, 1, 4), 0b110);
        assert_eq!(super::get_bits(1, 0, 0), 0);
    }
}
