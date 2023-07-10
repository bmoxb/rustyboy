use std::cmp;

use crate::bits::get_bits;
use crate::cartridge::Cartridge;

pub struct MBC1 {
    cart: Cartridge,
    /// Whether RAM is enabled or not (set by writing in range 0x000-0x1FFF).
    ram_enable: bool,
    /// The current ROM bank (this variable merges the two separate 5-bit and 2-bit registers).
    rom_bank: u8,
    /// RAM stored on the MBC (if any).
    ram: Option<Vec<u8>>,
}

impl MBC1 {
    pub fn new(cart: Cartridge, has_ram: bool, _has_battery: bool) -> Self {
        MBC1 {
            cart,
            ram_enable: false,
            rom_bank: 1,
            ram: has_ram.then(|| vec![0; 0x2000]), // TODO: handle larger RAM size
        }
    }
}

impl super::MemoryBankController for MBC1 {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3FFF => self.cart.read8(addr as usize),
            0x4000..=0x7FFF => self
                .cart
                .read8((self.rom_bank as usize * 0x4000) + addr as usize - 0x4000),
            0xA000..=0xBFFF => {
                if let Some(ram) = &self.ram {
                    if self.ram_enable {
                        // TODO: RAM banking
                        return ram[(addr - 0xA000) as usize];
                    }
                }
                0xFF
            }
            _ => 0,
        }
    }

    fn write8(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.ram_enable = get_bits(value, 0, 5) == 0xA, // lower 4 bits set to 0xA enables RAM
            0x2000..=0x3FFF => {
                let value = get_bits(value, 0, 6); // only lowest 5 bits can be modified here
                self.rom_bank = cmp::max(value, 1); // cannot be set to 0
            }
            0x4000..=0x5FFF => {}
            0x6000..=0x7FFF => {}
            0xA000..=0xBFFF => {
                if let Some(ram) = &mut self.ram {
                    if self.ram_enable {
                        // TODO: RAM banking
                        ram[(addr - 0xA000) as usize] = value;
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn read_
}
