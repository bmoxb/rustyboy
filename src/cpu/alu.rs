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
