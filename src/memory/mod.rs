use std::ops::{Index, IndexMut};

pub struct Memory {
    mem: [u8; 1000],
}

impl Memory {
    pub fn new() -> Self {
        Memory { mem: [0; 1000] }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        unimplemented!() // TODO
    }

    pub fn write16(&self, addr: u16, value: u16) {
        unimplemented!() // TODO
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, index: u16) -> &Self::Output {
        &self.mem[index as usize]
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.mem[index as usize]
    }
}
