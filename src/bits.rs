use std::ops::{BitAnd, BitOr, BitXor};

pub fn modify_bit<T>(value: T, bit: u8, set_to: bool) -> T
where
    T: BitOr<u8, Output = T> + BitAnd<u8, Output = T>,
{
    let mask = 1 << bit;
    if set_to {
        value | mask
    } else {
        value & !mask
    }
}

pub fn toggle_bit<T>(value: T, bit: u8) -> T
where
    T: BitXor<u8, Output = T>,
{
    let mask = 1 << bit;
    value ^ mask
}
