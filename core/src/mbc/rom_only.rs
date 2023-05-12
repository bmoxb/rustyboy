use crate::cartridge::Cartridge;

pub struct RomOnly {
    cart: Cartridge,
}

impl RomOnly {
    pub fn new(cart: Cartridge) -> Self {
        RomOnly { cart }
    }
}

impl super::MemoryBankController for RomOnly {
    fn read8(&self, addr: u16) -> u8 {
        self.cart.read8(addr)
    }

    fn write8(&mut self, addr: u16, value: u8) {
        log::warn!(
            "attempt made to write {value:#02X} to address {addr:#04X} of ROM-only cartridge"
        );
    }
}
