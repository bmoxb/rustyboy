use std::fmt;

use crate::bits::bit_accessors;
use crate::register_type;

macro_rules! reg_pair {
    ($get:ident, $set:ident, $self:ident, $x:ident, $y:ident) => {
        pub fn $get(&$self) -> u16 {
            u16::from_be_bytes([$self.$x, $self.$y])
        }

        pub fn $set(&mut $self, value: u16) {
            [$self.$x, $self.$y] = value.to_be_bytes();
        }
    };
}

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

    pub fn af(&self) -> u16 {
        u16::from_be_bytes([self.a, self.flags.0])
    }

    pub fn set_af(&mut self, value: u16) {
        [self.a, self.flags.0] = value.to_be_bytes();
        self.flags.0 &= 0xF0; // least sig nibble of F must always be 0b0000
    }

    reg_pair!(bc, set_bc, self, b, c);
    reg_pair!(de, set_de, self, d, e);
    reg_pair!(hl, set_hl, self, h, l);

    pub fn get16_with_sp(&self, index: u8) -> u16 {
        self.get16(index, self.sp)
    }

    pub fn get16_with_af(&self, index: u8) -> u16 {
        self.get16(index, self.af())
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

    fn set16(&mut self, index: u8, value: u16) {
        match index {
            0 => self.set_bc(value),
            1 => self.set_de(value),
            2 => self.set_hl(value),
            4 => {} // set in set16_with_sp and set16_with_af
            _ => panic!("16-bit register index {index} out of bounds"),
        }
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

register_type!(Flags);

impl Flags {
    pub fn new(c: bool, h: bool, n: bool, z: bool) -> Self {
        let mut f = Flags::default();
        f.set_carry(c);
        f.set_half_carry(h);
        f.set_subtraction(n);
        f.set_zero(z);
        f
    }

    bit_accessors!(4, carry, set_carry, toggle_carry);
    bit_accessors!(5, half_carry, set_half_carry);
    bit_accessors!(6, subtraction, set_subtraction);
    bit_accessors!(7, zero, set_zero);
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:#04X} (C={}, H={}, N={}, Z={})",
            self.0,
            self.carry(),
            self.half_carry(),
            self.subtraction(),
            self.zero()
        )
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

    #[test]
    fn get_set_flags() {
        let mut regs = Registers::default();
        regs.set_af(0b01010000);
        assert_eq!(regs.flags, Flags::new(true, false, true, false));

        let mut flags = Flags::default();

        assert_eq!(flags.0, 0);

        flags.set_zero(true);
        flags.set_half_carry(true);
        assert_eq!(flags, Flags::new(false, true, false, true));

        flags.toggle_carry();
        flags.set_half_carry(false);
        assert_eq!(flags, Flags::new(true, false, false, true));
    }
}
