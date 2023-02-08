use super::{Flag, Flags};

pub fn add8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let (result, carry) = x.overflowing_add(y);

    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, (x & 0x0F) + (y & 0x0F) > 0x0F)
        .set(Flag::Carry, carry);

    result
}

pub fn add16(flags: &mut Flags, x: u16, y: u16) -> u16 {
    let (result, carry) = x.overflowing_add(y);

    flags
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, (x & 0x7FF) + (x & 0x7FF) > 0x7FF) // carry from bit 11
        .set(Flag::Carry, carry);

    result
}

pub fn adc8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let c = flags.get(Flag::Carry) as u8;
    let result = x.wrapping_add(y).wrapping_add(c);

    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, (x & 0xF) + (y & 0xF) + c > 0xF)
        .set(Flag::Carry, (x as u16) + (y as u16) + (c as u16) > 0xFF);

    result
}

pub fn sub8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let (result, carry) = x.overflowing_sub(y);

    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, true)
        .set(Flag::HalfCarry, (x & 0xF) < (y & 0xF))
        .set(Flag::Carry, carry);

    result
}

pub fn sbc8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let c = flags.get(Flag::Carry) as u8;
    let result = x.wrapping_sub(y).wrapping_sub(c);

    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, true)
        .set(Flag::HalfCarry, (x & 0xF) < (y & 0xF) + c)
        .set(Flag::Carry, (x as u16) < (y as u16) + (c as u16));

    result
}

pub fn bitwise_and(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let result = x & y;

    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, true)
        .set(Flag::Carry, false);

    result
}

pub fn bitwise_or(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let result = x | y;

    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, false)
        .set(Flag::Carry, false);

    result
}

pub fn bitwise_xor(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let result = x ^ y;

    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, false)
        .set(Flag::Carry, false);

    result
}

pub fn bitwise_not(flags: &mut Flags, x: u8) -> u8 {
    flags
        .set(Flag::Subtraction, true)
        .set(Flag::HalfCarry, true);

    !x
}

pub fn rotate_left(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = x >> 7;
    let result = (x << 1) | carry_bit;
    set_rotation_flags(flags, result, carry_bit);
    result
}

pub fn rotate_left_through_carry_flag(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = x >> 7;
    let result = (x << 1) | (flags.get(Flag::Carry) as u8);
    set_rotation_flags(flags, result, carry_bit);
    result
}

pub fn rotate_right(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = x & 1;
    let result = (x >> 1) | (carry_bit << 7);
    set_rotation_flags(flags, result, carry_bit);
    result
}

pub fn rotate_right_through_carry_flag(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = x & 1;
    let result = (x >> 1) | ((flags.get(Flag::Carry) as u8) << 7);
    set_rotation_flags(flags, result, carry_bit);
    result
}

#[inline]
fn set_rotation_flags(flags: &mut Flags, result: u8, carry_bit: u8) {
    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, false)
        .set(Flag::Carry, carry_bit != 0);
}

// TODO: Tidy up flag setting?

pub fn test_bit(flags: &mut Flags, bit: u8, value: u8) {
    let mask = 1 << bit;
    let bit_set = (value & mask) != 0;

    flags
        .set(Flag::Zero, !bit_set)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, false);
}

pub fn set_bit(bit: u8, value: u8) -> u8 {
    let mask = 1 << bit;
    value | mask
}

pub fn reset_bit(bit: u8, value: u8) -> u8 {
    let mask = !(1 << bit);
    value & mask
}

pub fn swap_nibbles(flags: &mut Flags, value: u8) -> u8 {
    flags
        .set(Flag::Zero, value == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, false)
        .set(Flag::Carry, false);

    let upper = value >> 4;
    let lower = value & 0b1111;
    (lower << 4) + upper
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_testing() {
        let tests = vec![
            (0, 0, true),
            (0, 0xFF, false),
            (3, 0b1110111, true),
            (7, 0b10001000, false),
        ];

        let mut flags = Flags::default();

        for (bit, value, expected) in tests {
            test_bit(&mut flags, bit, value);

            assert_eq!(
                expected,
                flags.get(Flag::Zero),
                "bit {bit} of value {value} tested, expected {expected} zero flag"
            );
            assert!(!flags.get(Flag::Subtraction));
            assert!(!flags.get(Flag::HalfCarry));
        }
    }

    #[test]
    fn bit_setting() {
        assert_eq!(set_bit(0, 0), 1);
        assert_eq!(set_bit(0, 1), 1);
        assert_eq!(set_bit(2, 0b10011), 0b10111);
        assert_eq!(set_bit(7, 0b01111111), 0xFF);
    }

    #[test]
    fn bit_resetting() {
        assert_eq!(reset_bit(0, 0), 0);
        assert_eq!(reset_bit(0, 1), 0);
        assert_eq!(reset_bit(3, 0b1010), 0b10);
        assert_eq!(reset_bit(7, 0b10000000), 0);
    }

    #[test]
    fn nibble_swapping() {
        let mut flags = Flags::default();
        flags
            .set(Flag::Subtraction, true)
            .set(Flag::HalfCarry, true)
            .set(Flag::Carry, true);

        assert_eq!(swap_nibbles(&mut flags, 0xAB), 0xBA);
        assert!(!flags.get(Flag::Zero));
        assert!(!flags.get(Flag::Subtraction));
        assert!(!flags.get(Flag::HalfCarry));
        assert!(!flags.get(Flag::Carry));

        assert_eq!(swap_nibbles(&mut flags, 0), 0);
        assert!(flags.get(Flag::Zero));
    }
}
