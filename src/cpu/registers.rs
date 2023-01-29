#[derive(Default)]
pub struct Registers {
    pub a: u8,
    f: u8,
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
            7 => self.f,
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
            7 => &mut self.f,
            _ => panic!("8-bit register index {index} out of bounds"),
        };
        *reg = value;
    }

    pub fn get16(&self, index: u8) -> u16 {
        match index {
            0 => self.bc(),
            1 => self.de(),
            2 => self.hl(),
            3 => self.af(),
            _ => panic!("16-bit register index {index} out of bounds"),
        }
    }

    pub fn set16(&mut self, index: u8, value: u16) {
        unimplemented!()
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) + self.f as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value >> 8) & 0xFF) as u8;
        self.f = (value & 0xF0) as u8;
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

    pub fn flag(&self, flag: Flag) -> bool {
        (self.f & (1 << flag.bit())) != 0
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        let mask = 1 << flag.bit();
        if value {
            self.f |= mask;
        } else {
            self.f &= !mask;
        }
    }

    pub fn toggle_flag(&mut self, flag: Flag) {
        self.f ^= 1 << flag.bit();
    }
}

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
