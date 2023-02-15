#[cfg(test)]
mod tests;

use super::{Flag, Flags};

// 8-bit increase - affects zero, subtraction, and half carry flags.
pub fn inc8(flags: &mut Flags, x: u8) -> u8 {
    let result = x.wrapping_add(1);
    set_zero_subtraction_half_carry_add(flags, result, x, 1, 0);
    result
}

// 16-bit increase - affects no flags.
pub fn inc16(x: u16) -> u16 {
    x.wrapping_add(1)
}

// 8-bit decrease - affects zero, subtraction, an half carry flags.
pub fn dec8(flags: &mut Flags, x: u8) -> u8 {
    let result = x.wrapping_sub(1);
    set_zero_subtraction_half_carry_sub(flags, result, x, 1, 0);
    result
}

// 16-bit decrease - affects no flags.
pub fn dec16(x: u16) -> u16 {
    x.wrapping_sub(1)
}

// 8-bit addition - affects all flags.
pub fn add8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let (result, carry) = x.overflowing_add(y);

    set_zero_subtraction_half_carry_add(flags, result, x, y, 0);
    flags.set(Flag::Carry, carry);

    result
}

// 8-bit addition with carry - affects all flags.
pub fn adc8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let c = flags.get(Flag::Carry) as u8;
    let result = x.wrapping_add(y).wrapping_add(c);

    set_zero_subtraction_half_carry_add(flags, result, x, y, c);
    flags.set(Flag::Carry, (x as u16) + (y as u16) + (c as u16) > 0xFF);

    result
}

// 8-bit subtraction - affects all flags.
pub fn sub8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let (result, carry) = x.overflowing_sub(y);

    set_zero_subtraction_half_carry_sub(flags, result, x, y, 0);
    flags.set(Flag::Carry, carry);

    result
}

// 8-bit subtraction with carry - affects all flags.
pub fn sbc8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let c = flags.get(Flag::Carry) as u8;
    let result = x.wrapping_sub(y).wrapping_sub(c);

    set_zero_subtraction_half_carry_sub(flags, result, x, y, c);
    flags.set(Flag::Carry, (x as u16) < (y as u16) + (c as u16));

    result
}

// TODO: Specifically in the case of the instruction `ADD SP, n`, the half carry flag is set based on carry occurring
// at bit 3 instead of bit 11 - this is something that needs to be handled.
// Source: https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8

// 16-bit addition - affects subtraction, half carry, and carry flags.
pub fn add16(flags: &mut Flags, x: u16, y: u16) -> u16 {
    let (result, carry) = x.overflowing_add(y);

    flags
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, (x & 0x7FF) + (x & 0x7FF) > 0x7FF) // carry from bit 11
        .set(Flag::Carry, carry);

    result
}

pub fn bitwise_and(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let result = x & y;
    set_bitwise_flags(flags, result, true);
    result
}

pub fn bitwise_or(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let result = x | y;
    set_bitwise_flags(flags, result, false);
    result
}

pub fn bitwise_xor(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let result = x ^ y;
    set_bitwise_flags(flags, result, false);
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

pub fn shift_left(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = x >> 7;
    let result = x << 1;
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

pub fn shift_right_leave_msb(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = x & 1;
    let result = (x >> 1) + (x & 0x80);
    set_rotation_flags(flags, result, carry_bit);
    result
}

pub fn shift_right_clear_msb(flags: &mut Flags, x: u8) -> u8 {
    let carry_bit = x & 1;
    let result = x >> 1;
    set_rotation_flags(flags, result, carry_bit);
    result
}

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
    set_bitwise_flags(flags, value, false);
    let upper = value >> 4;
    let lower = value & 0b1111;
    (lower << 4) + upper
}

#[inline]
fn set_rotation_flags(flags: &mut Flags, result: u8, carry_bit: u8) {
    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, false)
        .set(Flag::Carry, carry_bit != 0);
}

#[inline]
fn set_bitwise_flags(flags: &mut Flags, result: u8, half_carry: bool) {
    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, half_carry)
        .set(Flag::Carry, false);
}

#[inline]
fn set_zero_subtraction_half_carry_add(flags: &mut Flags, result: u8, x: u8, y: u8, c: u8) {
    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, (x & 0xF) + (y & 0xF) + c > 0xF);
}

#[inline]
fn set_zero_subtraction_half_carry_sub(flags: &mut Flags, result: u8, x: u8, y: u8, c: u8) {
    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, true)
        .set(Flag::HalfCarry, (x & 0xF) < (y & 0xF) + c);
}
