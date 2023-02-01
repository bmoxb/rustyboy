use std::fmt;

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub flags: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn get8(&self, index: u8) -> u8 {
        match index {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => self.flags.0,
            7 => self.a,
            _ => panic!("8-bit register index {index} out of bounds"),
        }
    }

    pub fn set8(&mut self, index: u8, value: u8) {
        let reg = match index {
            0 => &mut self.b,
            1 => &mut self.c,
            2 => &mut self.d,
            3 => &mut self.e,
            4 => &mut self.h,
            5 => &mut self.l,
            6 => &mut self.flags.0,
            7 => &mut self.a,
            _ => panic!("8-bit register index {index} out of bounds"),
        };
        *reg = value;
    }

    pub fn get16_with_sp(&self, index: u8) -> u16 {
        self.get16(index, self.sp)
    }

    pub fn get16_with_af(&self, index: u8) -> u16 {
        self.get16(index, self.af())
    }

    fn get16(&self, index: u8, last: u16) -> u16 {
        match index {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => last,
            _ => panic!("16-bit register index {index} out of bounds"),
        }
    }

    pub fn set16_with_sp(&mut self, index: u8, value: u16) {
        if index == 3 {
            self.sp = value;
        } else {
            self.set16(index, value)
        }
    }

    pub fn set16_with_af(&mut self, index: u8, value: u16) {
        if index == 3 {
            self.set_af(value);
        } else {
            self.set16(index, value)
        }
    }

    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, (self.flags.0 & 0xF0)]) // least sig nibble of F must always be 0b0000
    }

    pub fn bc(&self) -> u16 {
        u16::from_be_bytes([self.b, self.c])
    }

    pub fn de(&self) -> u16 {
        u16::from_be_bytes([self.d, self.e])
    }

    pub fn hl(&self) -> u16 {
        u16::from_be_bytes([self.h, self.l])
    }

    pub fn set_af(&mut self, value: u16) {
        set_combined_reg(&mut self.a, &mut self.flags.0, value);
    }

    pub fn set_bc(&mut self, value: u16) {
        set_combined_reg(&mut self.b, &mut self.c, value);
    }

    pub fn set_de(&mut self, value: u16) {
        set_combined_reg(&mut self.d, &mut self.e, value);
    }

    pub fn set_hl(&mut self, value: u16) {
        set_combined_reg(&mut self.h, &mut self.l, value);
    }

    fn set16(&mut self, index: u8, value: u16) {
        match index {
            0 => self.set_bc(value),
            1 => self.set_de(value),
            2 => self.set_hl(value),
            4 => {} // set in set16_with_sp and set16_with_af
            _ => panic!("16-bit register index {index} out of bounds"),
        }
    }
}

#[inline]
fn set_combined_reg(high: &mut u8, low: &mut u8, value: u16) {
    *high = ((value >> 8) & 0xFF) as u8;
    *low = (value & 0xFF) as u8;
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "A: {:#04X}, F: {}, B: {:#04X}, C: {:#04X}, D: {:#04X}, E: {:#04X}, H: {:#04X}, L: {:#04X}, SP: {:#06X}, PC: {:#06X}",
            self.a,
            self.flags,
            self.b,
            self.c,
            self.d,
            self.e,
            self.h,
            self.l,
            self.sp,
            self.pc,
        )
    }
}

#[derive(Default)]
pub struct Flags(u8);

impl Flags {
    pub fn get(&self, flag: Flag) -> bool {
        (self.0 & (1 << flag.bit())) != 0
    }

    pub fn set(&mut self, flag: Flag, value: bool) -> &mut Self {
        let mask = 1 << flag.bit();
        if value {
            self.0 |= mask;
        } else {
            self.0 &= !mask;
        }
        self
    }

    pub fn toggle(&mut self, flag: Flag) -> &mut Self {
        self.0 ^= 1 << flag.bit();
        self
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#04X} (Z={}, N={}, H={}, C={})",
            self.0,
            self.get(Flag::Zero) as u8,
            self.get(Flag::Subtraction) as u8,
            self.get(Flag::HalfCarry) as u8,
            self.get(Flag::Carry) as u8,
        )
    }
}

#[derive(Clone, Copy)]
pub enum Flag {
    Zero,
    Subtraction,
    HalfCarry,
    Carry,
}

impl Flag {
    fn bit(&self) -> usize {
        match self {
            Flag::Zero => 7,
            Flag::Subtraction => 6,
            Flag::HalfCarry => 5,
            Flag::Carry => 4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_set_combined_registers() {
        let mut regs = Registers::default();

        macro_rules! test_combined_reg {
            ($x:ident, $y:ident, $xy:ident, $set_xy:ident) => {
                assert_eq!(regs.$xy(), 0);

                regs.$set_xy(0x1234);
                assert_eq!(regs.$x, 0x12);
                assert_eq!(regs.$y, 0x34);
                assert_eq!(regs.$xy(), 0x1234);

                regs.$x = 0xAB;
                regs.$y = 0xCD;
                assert_eq!(regs.$xy(), 0xABCD);
            };
        }

        test_combined_reg!(b, c, bc, set_bc);
        test_combined_reg!(d, e, de, set_de);
        test_combined_reg!(h, l, hl, set_hl);

        regs.set_af(0xFFFF);
        assert_eq!(regs.af(), 0xFFF0);
    }

    fn assert_flags(flags: &Flags, z: bool, n: bool, h: bool, c: bool) {
        assert_eq!(flags.get(Flag::Zero), z);
        assert_eq!(flags.get(Flag::Subtraction), n);
        assert_eq!(flags.get(Flag::HalfCarry), h);
        assert_eq!(flags.get(Flag::Carry), c);
    }

    #[test]
    fn get_set_flags() {
        let mut flags = Flags::default();

        assert_flags(&flags, false, false, false, false);

        flags.set(Flag::Zero, true).set(Flag::HalfCarry, true);
        assert_flags(&flags, true, false, true, false);

        flags.toggle(Flag::Carry).toggle(Flag::HalfCarry);
        assert_flags(&flags, true, false, false, true);

        let mut regs = Registers::default();
        regs.set_af(0b01010000);
        assert_flags(&regs.flags, false, true, false, true);
    }
}
