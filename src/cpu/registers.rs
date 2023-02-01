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
            6 => self.a,
            7 => self.flags.0,
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
            6 => &mut self.a,
            7 => &mut self.flags.0,
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
        ((self.a as u16) << 8) + self.flags.0 as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value >> 8) & 0xFF) as u8;
        self.flags.0 = (value & 0xFF) as u8;
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) + self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value >> 8) & 0xFF) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) + self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value >> 8) & 0xFF) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) + self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value >> 8) & 0xFF) as u8;
        self.l = (value & 0xFF) as u8;
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
