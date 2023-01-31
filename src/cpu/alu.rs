use super::{Flag, Flags};

pub fn add8(flags: &mut Flags, x: u8, y: u8, include_carry: bool) -> u8 {
    let c = (include_carry && flags.get(Flag::Carry)) as u8;

    let result = x.wrapping_add(y).wrapping_add(c);

    flags.set(Flag::Zero, result == 0);
    flags.set(Flag::Subtraction, false);
    flags.set(Flag::HalfCarry, (x & 0x0F) + (y & 0x0F) + c > 0x0F);
    flags.set(Flag::Carry, (x as u16) + (y as u16) + (c as u16) > 0xFF);

    result
}

pub fn sub8(flags: &mut Flags, x: u8, y: u8, include_carry: bool) -> u8 {
    let c = (include_carry && flags.get(Flag::Carry)) as u8;

    let result = x.wrapping_sub(y).wrapping_sub(c);

    flags.set(Flag::Zero, result == 0);
    flags.set(Flag::Subtraction, true);
    flags.set(Flag::HalfCarry, (x & 0x0F) < (y & 0x0F) + c);
    flags.set(Flag::Carry, (x as u16) < (y as u16) + (c as u16));

    result
}
