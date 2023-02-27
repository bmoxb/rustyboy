use crate::bits::{bit_accessors, get_bits, modify_bits};
use crate::register_type;
use crate::Display;

const VRAM_SIZE: usize = 0x2000;

const HBLANK_PERIOD: usize = 204;
const VBLANK_PERIOD: usize = 456; // single line
const SEARCHING_OAM_PERIOD: usize = 80;
const TRANSFERRING_DATA_PERIOD: usize = 172;

pub struct Gpu {
    display: Box<dyn Display>,
    pub vram: [u8; VRAM_SIZE],
    pub lcd_control: LcdControlRegister,
    pub lcd_status: LcdStatusRegister,
    pub viewport_y: u8,
    pub viewport_x: u8,
    pub lcd_y: u8,
    pub ly_compare: u8,
    pub bg_palette_data: u8,
    pub obj_palette_0_data: u8,
    pub obj_palette_1_data: u8,
    pub window_y: u8,
    pub window_x: u8,
    clock: usize,
    scanline: u8,
}

impl Gpu {
    pub fn new(display: Box<dyn Display>) -> Self {
        Gpu {
            display,
            vram: [0; VRAM_SIZE],
            lcd_control: LcdControlRegister(0x91),
            lcd_status: LcdStatusRegister(0x81),
            viewport_y: 0,
            viewport_x: 0,
            lcd_y: 0x91,
            ly_compare: 0,
            bg_palette_data: 0xFC,
            obj_palette_0_data: 0,
            obj_palette_1_data: 0,
            window_y: 0,
            window_x: 0,
            clock: 0,
            scanline: 0,
        }
    }

    pub fn update(&mut self, t_cycles: usize) {
        self.clock += t_cycles;

        match self.lcd_status.status() {
            // horizontal blank
            LcdStatus::HBlank => {
                if self.clock >= HBLANK_PERIOD {
                    self.clock -= HBLANK_PERIOD;

                    self.scanline += 1;

                    let next_status = if self.scanline == 143 {
                        self.display.swap_buffers();
                        LcdStatus::HBlank
                    } else {
                        LcdStatus::SearchingOAM
                    };

                    self.lcd_status.set_status(next_status);
                }
            }

            // vertical blank
            LcdStatus::VBlank => {
                if self.clock >= VBLANK_PERIOD {
                    self.clock -= VBLANK_PERIOD;

                    self.scanline += 1;

                    // if 10 lines done since HBlank (i.e., 10 * VBLANK_PERIOD ticks elapsed)
                    if self.scanline > 153 {
                        self.scanline = 0;
                        self.lcd_status.set_status(LcdStatus::SearchingOAM);
                    }
                }
            }

            // scanline (accessing OAM)
            LcdStatus::SearchingOAM => {
                if self.clock >= SEARCHING_OAM_PERIOD {
                    self.clock -= SEARCHING_OAM_PERIOD;
                    self.lcd_status
                        .set_status(LcdStatus::TransferringDataToController);
                }
            }

            // scanline (accessing VRAM)
            LcdStatus::TransferringDataToController => {
                if self.clock >= TRANSFERRING_DATA_PERIOD {
                    self.clock -= TRANSFERRING_DATA_PERIOD;
                    self.lcd_status.set_status(LcdStatus::HBlank);

                    self.display.write_scanline();
                }
            }
        }
    }
}

register_type!(LcdControlRegister);

#[allow(dead_code)]
impl LcdControlRegister {
    bit_accessors!(0, bg_and_window_enable);
    bit_accessors!(1, obj_enable);
    bit_accessors!(2, obj_size);
    bit_accessors!(3, bg_tile_map_area);
    bit_accessors!(4, bg_and_window_tile_data_area);
    bit_accessors!(5, window_enable);
    bit_accessors!(6, window_tile_map_area);
    bit_accessors!(7, lcd_enable);
}

register_type!(LcdStatusRegister);

#[allow(dead_code)]
impl LcdStatusRegister {
    fn status(&self) -> LcdStatus {
        match get_bits(self.0, 0, 2) {
            0 => LcdStatus::HBlank,
            1 => LcdStatus::VBlank,
            2 => LcdStatus::SearchingOAM,
            3 => LcdStatus::TransferringDataToController,
            _ => unreachable!(),
        }
    }
    fn set_status(&mut self, status: LcdStatus) {
        self.0 = modify_bits(self.0, 0, 2, status as u8);
    }

    bit_accessors!(2, ly_and_lyc_are_equal);
}

enum LcdStatus {
    HBlank,
    VBlank,
    SearchingOAM,
    TransferringDataToController,
}
