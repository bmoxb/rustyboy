use std::cmp;

use crate::bits::{get_bit, get_bits, modify_bits};
use crate::cartridge::Cartridge;

pub struct MBC1 {
    /// The game cartridge.
    cart: Cartridge,
    /// Whether RAM is enabled or not (set by writing in range 0x000-0x1FFF).
    ram_enable: bool,
    /// The current ROM bank (this variable merges the two separate 5-bit (address range 0x2000-0x3FFF) and 2-bit
    /// (address range 0x4000-0x5FFF) registers).
    rom_bank: u8,
    /// RAM stored on the MBC (if any).
    ram: Option<Vec<u8>>,
    /// The current RAM bank (from bank 0 to 3 inclusive). Set by writing in address range 0x4000-0x5FFF.
    ram_bank: u8,
    /// The current banking mode (either 'simple' or 'advanced' - see [`BankingMode`] documentation for an explanation
    /// of these).
    banking_mode: BankingMode,
}

impl MBC1 {
    pub fn new(cart: Cartridge, has_ram: bool, _has_battery: bool) -> Self {
        let ram = has_ram.then(|| vec![0; cart.ram_size()]);
        MBC1 {
            cart,
            ram_enable: false,
            rom_bank: 1,
            ram,
            ram_bank: 0,
            banking_mode: BankingMode::Simple,
        }
    }
}

impl super::MemoryBankController for MBC1 {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            // ROM bank 0 (or bank 32, 64, or 96 in advanced banking mode)
            0x0000..=0x3FFF => match self.banking_mode {
                BankingMode::Simple => self.read_rom_bank(0, addr),
                BankingMode::Advanced => {
                    let two_bits = get_bits(self.rom_bank, 5, 7);
                    self.read_rom_bank(two_bits * 0x20, addr)
                }
            },

            // ROM bank 1-127
            0x4000..=0x7FFF => self.read_rom_bank(self.rom_bank, addr - 0x4000),

            // RAM bank 0-3
            0xA000..=0xBFFF => {
                if let Some(ram) = &self.ram {
                    if self.ram_enable {
                        return ram[(addr - 0xA000 + self.ram_bank_offset()) as usize];
                    }
                }
                0xFF
            }

            _ => 0,
        }
    }

    fn write8(&mut self, addr: u16, value: u8) {
        match addr {
            // RAM enable/disable
            0x0000..=0x1FFF => self.ram_enable = get_bits(value, 0, 5) == 0xA, // lower 4 bits set to 0xA enables RAM

            // select ROM bank
            0x2000..=0x3FFF => {
                let value = get_bits(value, 0, 5); // only lowest 5 bits can be modified here
                self.rom_bank = cmp::max(value, 1); // cannot be set to 0
            }

            // select RAM bank or upper bits of ROM bank number
            0x4000..=0x5FFF => {
                let two_bits = get_bits(value, 0, 2);
                self.ram_bank = two_bits;
                self.rom_bank = modify_bits(self.rom_bank, 5, 7, two_bits);
            }

            // select banking mode
            0x6000..=0x7FFF => {
                self.banking_mode = if get_bit(value, 0) {
                    BankingMode::Advanced
                } else {
                    BankingMode::Simple
                };
            }

            // write RAM bank 0-3
            0xA000..=0xBFFF => {
                let bank_offset = self.ram_bank_offset();
                if let Some(ram) = &mut self.ram {
                    if self.ram_enable {
                        ram[(addr - 0xA000 + bank_offset) as usize] = value;
                    }
                }
            }

            _ => {}
        }
    }
}

impl MBC1 {
    fn read_rom_bank(&self, bank: u8, offset: u16) -> u8 {
        self.cart.read8((bank as usize * 0x4000) + offset as usize)
    }

    fn ram_bank_offset(&self) -> u16 {
        match self.banking_mode {
            BankingMode::Simple => 0,
            BankingMode::Advanced => self.ram_bank as u16 * 0x2000,
        }
    }
}

enum BankingMode {
    /// In simple banking mode, access to RAM and ROM in memory range 0x000-0x3FFF is locked to
    /// their respective 0th banks. ROM banks 1 to 127 can still be access through memory range
    /// 0x4000-0x7FFF even in this mode.
    Simple,
    /// In advanced banking mode, the 2-bit register in memory range 0x4000-0x5FFF can be used to
    /// access RAM banks 1-3 but also to access RAM banks 32, 64, and 96 through the memory range
    /// 0x000-0x1FFF.
    Advanced,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mbc::MemoryBankController;

    #[test]
    fn read_rom_no_banking() {
        let mut data = vec![0; 0x4000]; // 16 KiB
        data[0x0] = 0xA;
        data[0x100] = 0xB;
        data[0x3FFF] = 0xC;
        let mbc = MBC1::new(Cartridge::from_data(data), false, false);
        assert_eq!(mbc.read8(0x0), 0xA);
        assert_eq!(mbc.read8(0x100), 0xB);
        assert_eq!(mbc.read8(0x3FFF), 0xC);
    }

    #[test]
    fn read_rom_banks_5_bit_bank_number() {
        let mut data = vec![0; 0x10000]; // 64 KiB
        data[0x4000] = 0xA; // set test value in bank 1
        data[0x8100] = 0xB; // set test value in bank 2
        let mut mbc = MBC1::new(Cartridge::from_data(data), false, false);
        mbc.write8(0x2000, 1); // select bank 1
        assert_eq!(mbc.read8(0x4000), 0xA);
        mbc.write8(0x2500, 0); // setting to 0 should still use bank 1
        assert_eq!(mbc.read8(0x4000), 0xA);
        mbc.write8(0x3000, 0b11100010); // higher bits discarded - select bank 2
        assert_eq!(mbc.read8(0x4100), 0xB);
    }

    #[test]
    fn read_rom_banks_7_bit_bank_number() {
        let mut data = vec![0; 0x200000]; // 2048 KiB
        data[0x84000] = 0xA; // write a value in bank 33
        data[0x1FC000] = 0xB; // write a value in bank 127
        let mut mbc = MBC1::new(Cartridge::from_data(data), false, false);

        // access bank 33
        mbc.write8(0x2000, 1);
        mbc.write8(0x4000, 1);
        assert_eq!(mbc.read8(0x4000), 0xA);

        // access bank 127
        mbc.write8(0x2000, 0xFF);
        mbc.write8(0x4000, 0xFF);
        assert_eq!(mbc.read8(0x4000), 0xB);
    }

    #[test]
    fn read_rom_banks_advanced_banking_mode() {
        let mut data = vec![0; 0x200000]; // 2048 KiB
        data[0x80000] = 0xA; // write a value in bank 32
        data[0x84000] = 0xB; // write a value in bank 33

        let mut mbc = MBC1::new(Cartridge::from_data(data), false, false);
        mbc.write8(0x6000, 1); // advanced banking mode

        // select bank 33 when reading range 0x4000-0x7FFF
        mbc.write8(0x2000, 1);
        mbc.write8(0x4000, 1); // due to advanced bank mode, this also selects bank 32 in range 0x0000-0x3FFF

        assert_eq!(mbc.read8(0x4000), 0xB); // in bank 33
        assert_eq!(mbc.read8(0x0000), 0xA); // in bank 32, thanks to advanced banking
    }

    #[test]
    fn read_ram_no_banking() {
        let mut data = vec![0; 0x200];
        data[0x149] = 2; // 1 RAM bank
        let mut mbc = MBC1::new(Cartridge::from_data(data), true, false);
        mbc.write8(0, 0xA); // enable RAM
        mbc.write8(0xA000, 0xAB);
        assert_eq!(mbc.read8(0xA000), 0xAB);
        mbc.write8(0xBFFF, 0xCD);
        assert_eq!(mbc.read8(0xBFFF), 0xCD);
    }

    #[test]
    fn read_ram_banks() {
        let mut data = vec![0; 0x200];
        data[0x149] = 3; // 4 RAM banks
        let mut mbc = MBC1::new(Cartridge::from_data(data), true, false);
        mbc.write8(0, 0xA); // enable RAM
        mbc.write8(0x6000, 1); // RAM banking mode

        mbc.write8(0x4000, 1); // select bank 1
        mbc.write8(0xA000, 0xA);
        assert_eq!(mbc.read8(0xA000), 0xA);

        mbc.write8(0x5000, 2); // select bank 2
        assert_ne!(mbc.read8(0xA000), 0xA); // ensure value set in bank 1 not carried across
        mbc.write8(0xA500, 0xB);
        assert_eq!(mbc.read8(0xA500), 0xB);

        mbc.write8(0x5FFF, 3); // select bank 3
        assert_ne!(mbc.read8(0xA500), 0xB); // ensure value set in bank 2 not carried across
        mbc.write8(0xB000, 0xC);
        assert_eq!(mbc.read8(0xB000), 0xC);

        mbc.write8(0x4000, 0b11100); // select bank 0 (only the 2 least significant bits relevant)
        assert_ne!(mbc.read8(0xB000), 0xC); // ensure value set in bank 3 not carried across
        mbc.write8(0xBFFF, 0xC);
        assert_eq!(mbc.read8(0xBFFF), 0xC);
    }

    #[test]
    fn try_read_ram_banks_simple_banking_mode() {
        let mut data = vec![0; 0x200];
        data[0x149] = 3; // 4 RAM banks
        let mut mbc = MBC1::new(Cartridge::from_data(data), true, false);
        mbc.write8(0, 0xA); // enable RAM
        mbc.write8(0x6000, 0); // simple banking mode (i.e., lock to bank 0)

        // write value to bank 0
        mbc.write8(0x4000, 0);
        mbc.write8(0xA000, 0xAB);

        // In simple banking mode, the RAM bank register is ignored - we can only read/write RAM
        // bank 0.
        mbc.write8(0x4000, 1); // attempt to select bank 1
        assert_eq!(mbc.read8(0xA000), 0xAB); // check that we're in fact still in bank 0

        mbc.write8(0x7000, 1); // RAM banking mode (now we can actually access bank 1)
        assert_ne!(mbc.read8(0xA000), 0xAB); // value written to bank 0 shouldn't be present here
    }

    #[test]
    fn enable_disable_ram() {
        let mut data = vec![0; 0x200];
        data[0x149] = 2; // 1 RAM bank
        let mut mbc = MBC1::new(Cartridge::from_data(data), true, false);

        mbc.write8(0, 0xA); // enable RAM
        mbc.write8(0xA000, 123);
        assert_eq!(mbc.read8(0xA000), 123);

        // Technically read values aren't guarenteed to be 0xFF when RAM is disabled on real hardware, but for the sake
        // of this emulator we will assume that they are.
        mbc.write8(0, 0); // disable RAM
        assert_eq!(mbc.read8(0xA000), 0xFF);

        mbc.write8(0, 0xEA); // enable RAM (0xA in lower 4 bits, top 4 bits are ignored).
        mbc.write8(0xBBBB, 101);
        assert_eq!(mbc.read8(0xBBBB), 101);

        mbc.write8(0, 0xAE); // disable RAM (0xA not in lower 4 bits)
        assert_eq!(mbc.read8(0xBBBB), 0xFF);

        mbc.write8(0x1FFF, 0xA); // enable RAM (write in upper bound of register range)
        mbc.write8(0xBFFF, 123);
        assert_eq!(mbc.read8(0xBFFF), 123);
    }
}
