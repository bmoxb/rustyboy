mod mbc1;
mod rom_only;

use crate::cartridge::{Cartridge, CartridgeType};

pub fn from_cartridge(cart: Cartridge) -> Option<Box<dyn MemoryBankController>> {
    match cart.cart_type() {
        CartridgeType::RomOnly => Some(Box::new(rom_only::RomOnly::new(cart))),
        CartridgeType::MBC1 { ram, battery } => Some(Box::new(mbc1::MBC1::new(cart, ram, battery))),
        CartridgeType::MBC3 {
            timer: _,
            ram: _,
            battery: _,
        } => unimplemented!(),
        CartridgeType::Unsupported(_) => None, // TODO: proper error type
    }
}

pub trait MemoryBankController {
    fn read8(&self, addr: u16) -> u8;
    fn write8(&mut self, addr: u16, value: u8);
}
