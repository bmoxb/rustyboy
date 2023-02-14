#[cfg(test)]
mod tests;

use super::{Flag, Flags};

// TODO: INC and DEC instructions do not affect flags, therefore need separate ALU functions.

pub fn add8(flags: &mut Flags, x: u8, y: u8) -> u8 {
    let (result, carry) = x.overflowing_add(y);

    flags
        .set(Flag::Zero, result == 0)
        .set(Flag::Subtraction, false)
        .set(Flag::HalfCarry, (x & 0x0F) + (y & 0x0F) > 0x0F)
        .set(Flag::Carry, carry);

    result
}

// TODO: Specifically in the case of the instruction `ADD SP, n`, the half carry flag is set based on carry occurring
// at bit 3 instead of bit 11 - this is something that needs to be handled.
// Source: https://stackoverflow.com/questions/57958631/game-boy-half-carry-flag-and-16-bit-instructions-especially-opcode-0xe8

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
