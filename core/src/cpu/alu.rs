use crate::bits::{get_bit, modify_bit};

use super::Flags;

/// 8-bit increase - affects zero, subtraction, and half carry flags.
pub fn inc8(flags: &mut Flags, x: u8) -> u8 {
    let result = x.wrapping_add(1);
    set_zero_subtraction_half_carry_add(flags, result, x, 1, 0);
    result
}

/// 16-bit increase - affects no flags.
pub fn inc16(x: u16) -> u16 {
    x.wrapping_add(1)
}

/// 8-bit decrease - affects zero, subtraction, an half carry flags.
pub fn dec8(flags: &mut Flags, x: u8) -> u8 {
    let result = x.wrapping_sub(1);
    set_zero_subtraction_half_carry_sub(flags, result, x, 1, 0);
    result
}

/// 16-bit decrease - affects no flags.
pub fn dec16(x: u16) -> u16 {
    x.wrapping_sub(1)
}

/// 8-bit addition - affects all flags.
pub fn add8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let (result, carry) = x.overflowing_add(y);

    set_zero_subtraction_half_carry_add(flags, result, x, y, 0);
    flags.set_carry(carry);

    result
}

/// 8-bit addition with carry - affects all flags.
pub fn adc8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let c = flags.carry() as u8;
    let result = x.wrapping_add(y).wrapping_add(c);

    set_zero_subtraction_half_carry_add(flags, result, x, y, c);
    flags.set_carry((x as u16) + (y as u16) + (c as u16) > 0xFF);

    result
}

/// 8-bit subtraction - affects all flags.
pub fn sub8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let (result, carry) = x.overflowing_sub(y);

    set_zero_subtraction_half_carry_sub(flags, result, x, y, 0);
    flags.set_carry(carry);

    result
}

/// 8-bit subtraction with carry - affects all flags.
pub fn sbc8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let c = flags.carry() as u8;
    let result = x.wrapping_sub(y).wrapping_sub(c);

    set_zero_subtraction_half_carry_sub(flags, result, x, y, c);
    flags.set_carry((x as u16) < (y as u16) + (c as u16));

    result
}

/// 16-bit addition - affects subtraction, half carry, and carry flags.
pub fn add16(flags: &mut Flags, x: u16, y: u16) -> u16 {
    let (result, carry) = x.overflowing_add(y);

    flags.set_subtraction(false);
    flags.set_half_carry((((x & 0xFFF) + (y & 0xFFF)) & 0x1000) == 0x1000); // carry from bit 11
    flags.set_carry(carry);

    result
}

/// 16-bit addition except half-carry occurs at bit 3 and the zero flag is set - affects all flags.
/// https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8
pub fn add16_with_signed_byte_operand(flags: &mut Flags, x: u16, y: u8) -> u16 {
    let y = y as i8 as i16 as u16;

    flags.set_zero(false);
    flags.set_subtraction(false);
    flags.set_half_carry((x & 0xF) + (y & 0xF) > 0xF);
    flags.set_carry((x & 0xFF) + (y & 0xFF) > 0xFF);

    x.wrapping_add(y)
}

/// Bitwise 'and' operation (x & y) - affects all flags.
pub fn bitwise_and(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let result = x & y;
    set_bitwise_flags(flags, result, true);
    result
}

/// Bitwise 'or' operation (x | y) - affects all flags.
pub fn bitwise_or(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let result = x | y;
    set_bitwise_flags(flags, result, false);
    result
}

/// Bitwise 'exclusive or' operation (x ^ y) - affects all flags.
pub fn bitwise_xor(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let result = x ^ y;
    set_bitwise_flags(flags, result, false);
    result
}

/// Bitwise 'not'/complement (!x) - affects subtraction, and half carry flags.
pub fn bitwise_not(flags: &mut Flags, x: u8) -> u8 {
    flags.set_subtraction(true);
    flags.set_half_carry(true);
    !x
}

/// Rotate left - affects all flags.
pub fn rotate_left(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = get_bit(x, 7);

    let mut shifted = x << 1;
    shifted = modify_bit(shifted, 0, carry_bit);

    set_rotation_flags(flags, shifted, carry_bit);

    shifted
}

/// Rotate left using the carry flag as the new least-significant bit - affects all flags.
pub fn rotate_left_through_carry_flag(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = get_bit(x, 7);

    let mut shifted = x << 1;
    shifted = modify_bit(shifted, 0, flags.carry());

    set_rotation_flags(flags, shifted, carry_bit);

    shifted
}

/// Shift left - affects all flags.
pub fn shift_left(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = get_bit(x, 7);
    let shifted = x << 1;
    set_rotation_flags(flags, shifted, carry_bit);
    shifted
}

/// Rotate right - affects all flags.
pub fn rotate_right(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = get_bit(x, 0);

    let mut shifted = x >> 1;
    shifted = modify_bit(shifted, 7, carry_bit);

    set_rotation_flags(flags, shifted, carry_bit);

    shifted
}

/// Rotate right using the carry flag as the new most-significant bit - affects all flags.
pub fn rotate_right_through_carry_flag(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = get_bit(x, 0);

    let mut shifted = x >> 1;
    shifted = modify_bit(shifted, 7, flags.carry());

    set_rotation_flags(flags, shifted, carry_bit);

    shifted
}

/// Shift right but keep the most-significant bit from before the shift - affects all flags.
pub fn shift_right_leave_msb(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = get_bit(x, 0);
    let most_sig_bit = get_bit(x, 7);

    let mut shifted = x >> 1;
    shifted = modify_bit(shifted, 7, most_sig_bit);

    set_rotation_flags(flags, shifted, carry_bit);

    shifted
}

/// Shift right and set the most-significant bit to 0 - affects all flags.
pub fn shift_right_clear_msb(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = get_bit(x, 0);
    let shifted = x >> 1;
    set_rotation_flags(flags, shifted, carry_bit);
    shifted
}

/// Check if the given bit is set in the given value and then set zero, subtraction, and half carry flags accordingly.
pub fn test_bit(flags: &mut Flags, value: u8, bit: u8) {
    let set = get_bit(value, bit);
    flags.set_zero(!set);
    flags.set_subtraction(false);
    flags.set_half_carry(true);
}

/// Swap the nibbles in the given byte - affects all flags.
pub fn swap_nibbles(flags: &mut Flags, value: u8) -> u8 {
    set_bitwise_flags(flags, value, false);
    let upper = value >> 4;
    let lower = value & 0x0F;
    (lower << 4) + upper
}

pub fn daa(flags: &mut Flags, mut value: u8) -> u8 {
    // This implementation is based on https://ehaskins.com/2018-01-30%20Z80%20DAA/ so thank you to the author of that
    // post!

    let mut correction = 0;
    let mut carry = false;

    if flags.half_carry() || (!flags.subtraction() && (value & 0xF) > 9) {
        correction |= 0x6;
    }

    if flags.carry() || (!flags.subtraction() && value > 0x99) {
        correction |= 0x60;
        carry = true;
    }

    if flags.subtraction() {
        value = value.wrapping_sub(correction);
    } else {
        value = value.wrapping_add(correction);
    }

    flags.set_zero(value == 0);
    flags.set_half_carry(false);
    flags.set_carry(carry);

    value
}

#[inline]
fn set_rotation_flags(flags: &mut Flags, result: u8, carry: bool) {
    flags.set_zero(result == 0);
    flags.set_subtraction(false);
    flags.set_half_carry(false);
    flags.set_carry(carry);
}

#[inline]
fn set_bitwise_flags(flags: &mut Flags, result: u8, half_carry: bool) {
    flags.set_zero(result == 0);
    flags.set_subtraction(false);
    flags.set_half_carry(half_carry);
    flags.set_carry(false);
}

#[inline]
fn set_zero_subtraction_half_carry_add(flags: &mut Flags, result: u8, x: u8, y: u8, c: u8) {
    flags.set_zero(result == 0);
    flags.set_subtraction(false);
    flags.set_half_carry((x & 0xF) + (y & 0xF) + c > 0xF);
}

#[inline]
fn set_zero_subtraction_half_carry_sub(flags: &mut Flags, result: u8, x: u8, y: u8, c: u8) {
    flags.set_zero(result == 0);
    flags.set_subtraction(true);
    flags.set_half_carry((x & 0xF) < (y & 0xF) + c);
}

#[cfg(test)]
mod tests {
    use super::*;

    // This macro calls a given arithmetic function (e.g., add8) on the two operands and then assert the result and the
    // expected flag values.
    macro_rules! assert_arithmetic {
        ($f:ident, $flags:ident, $x:expr, $y:expr, $expected:expr, $c_flag:expr, $h_flag:expr, $n_flag:expr, $z_flag:expr) => {
            let result = $f(&mut $flags, $x, $y);

            assert_eq!(
                result,
                $expected,
                "{}({}, {}) gave {} but expected {}",
                stringify!($f),
                $x,
                $y,
                result,
                $expected,
            );

            assert_eq!(
                $flags,
                Flags::new($c_flag, $h_flag, $n_flag, $z_flag),
                "{}({}, {}) did not set flags correctly",
                stringify!($f),
                $x,
                $y
            );
        };
    }

    macro_rules! assert_bitwise {
    ($func:ident, $x:expr, $y:expr, $expected:expr, $h_flag:literal) => {
        let mut flags = Flags::new(true, true, true, false); // set flags that need to be reset to ensure that they are

        let result = $func(&mut flags, $x, $y);
        assert_eq!(
            result,
            $expected,
            "{}({}, {}) gave {:b} but expected {:b}",
            stringify!($func),
            $x,
            $y,
            result,
            $expected
        );

        assert_eq!(flags, Flags::new(false, $h_flag, false, result == 0));
    };
}

    #[test]
    fn addition_8_bit() {
        let mut flags = Flags::default();

        assert_arithmetic!(add8, flags, 0, 0, 0, false, false, false, true);
        assert_arithmetic!(add8, flags, 15, 1, 16, false, true, false, false);
        assert_arithmetic!(add8, flags, 255, 1, 0, true, true, false, true);
        assert_arithmetic!(add8, flags, 200, 200, 144, true, true, false, false);

        flags.set_carry(false);
        assert_arithmetic!(adc8, flags, 15, 1, 16, false, true, false, false);
        assert_arithmetic!(adc8, flags, 200, 200, 144, true, true, false, false);
    }

    #[test]
    fn addition_8_bit_carry() {
        let mut flags = Flags::default();

        flags.set_carry(true);
        assert_arithmetic!(adc8, flags, 0, 0, 1, false, false, false, false);
        flags.set_carry(true);
        assert_arithmetic!(adc8, flags, 14, 1, 16, false, true, false, false);
        flags.set_carry(true);
        assert_arithmetic!(adc8, flags, 200, 55, 0, true, true, false, true);
    }

    #[test]
    fn subtraction() {
        let mut flags = Flags::default();

        assert_arithmetic!(sub8, flags, 0, 0, 0, false, false, true, true);
        assert_arithmetic!(sub8, flags, 100, 10, 90, false, true, true, false);
        assert_arithmetic!(sub8, flags, 50, 100, 206, true, true, true, false);

        flags.set_carry(false);
        assert_arithmetic!(sbc8, flags, 0, 0, 0, false, false, true, true);
        assert_arithmetic!(sbc8, flags, 50, 100, 206, true, true, true, false);
    }

    #[test]
    fn subtraction_carry() {
        let mut flags = Flags::default();
        flags.set_carry(true);
        assert_arithmetic!(sbc8, flags, 1, 0, 0, false, false, true, true);
    }

    #[test]
    fn addition_16_bit() {
        let mut flags = Flags::default();

        // we expect add16 to not change the zero flag even if the result is zero, so we set the flag now and ensure it
        // remains set
        flags.set_zero(true);

        assert_arithmetic!(add16, flags, 0, 0, 0, false, false, false, true);
        assert_arithmetic!(add16, flags, 200, 300, 500, false, false, false, true);
        assert_arithmetic!(add16, flags, 4095, 5, 4100, false, true, false, true);
        assert_arithmetic!(add16, flags, 30500, 60500, 25464, true, true, false, true);
    }

    #[test]
    fn bitwise_operations() {
        assert_bitwise!(bitwise_and, 0, 0, 0, true);
        assert_bitwise!(bitwise_and, 0b1001010, 0b1000101, 0b1000000, true);
        assert_bitwise!(bitwise_or, 0b1010, 0b100110, 0b101110, false);
        assert_bitwise!(bitwise_xor, 0b11111, 0b10100, 0b01011, false);
    }

    #[test]
    fn complement() {
        let mut flags = Flags::new(true, false, false, true); // set zero and carry to ensure they're unaffected
        assert_eq!(bitwise_not(&mut flags, 0b101010), 0b11010101);
        assert_eq!(flags, Flags::new(true, true, true, true));
    }

    #[test]
    fn nibble_swapping() {
        let mut flags = Flags::new(true, true, true, false); // set carry, half carry, subtraction to ensure they're cleared

        assert_eq!(swap_nibbles(&mut flags, 0xAB), 0xBA);
        assert_eq!(flags, Flags::default()); // ensure all cleared

        assert_eq!(swap_nibbles(&mut flags, 0), 0);
        assert!(flags.zero());
    }

    #[test]
    fn rotation() {
        let mut flags = Flags::default();

        assert_eq!(rotate_left(&mut flags, 0b11001100), 0b10011001);
        assert_eq!(flags, Flags::new(true, false, false, false));
    }
}
