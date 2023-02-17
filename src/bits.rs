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
