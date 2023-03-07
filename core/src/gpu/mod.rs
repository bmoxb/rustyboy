mod palettes;
pub mod vram;

use palettes::*;
use vram::VideoRam;

use crate::bits::{bit_accessors, get_bits, modify_bits};
use crate::cycles::TCycles;
use crate::interrupts::{Interrupt, Interrupts};
use crate::screen::Screen;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

const HBLANK_PERIOD: TCycles = TCycles(204);
const VBLANK_PERIOD: TCycles = TCycles(456); // single line
const SEARCHING_OAM_PERIOD: TCycles = TCycles(80);
const TRANSFERRING_DATA_PERIOD: TCycles = TCycles(172);

pub struct Gpu {
    screen: Box<dyn Screen>,
    pub vram: VideoRam,
    pub lcd_control: LcdControlRegister,
    pub lcd_status: LcdStatusRegister,
    pub viewport_y: u8,
    pub viewport_x: u8,
    pub lcd_y: u8,
    pub ly_compare: u8,
    pub bg_palette_data: BackgroundPalette,
    pub obj_palette_0_data: ObjectPalette,
    pub obj_palette_1_data: ObjectPalette,
    pub window_y: u8,
    pub window_x: u8,
    clock: TCycles,
}

impl Gpu {
    pub fn new(screen: Box<dyn Screen>) -> Self {
        Gpu {
            screen,
            vram: VideoRam::new(),
            lcd_control: LcdControlRegister(0x91),
            lcd_status: LcdStatusRegister(0x81),
            viewport_y: 0,
            viewport_x: 0,
            lcd_y: 0x91,
            ly_compare: 0,
            bg_palette_data: BackgroundPalette(0xFC),
            obj_palette_0_data: ObjectPalette(0),
            obj_palette_1_data: ObjectPalette(0),
            window_y: 0,
            window_x: 0,
            clock: TCycles(0),
        }
    }

    pub fn update(&mut self, interrupts: &mut Interrupts, cycles: TCycles) {
        self.clock += cycles;

        self.compare_ly_lyc(interrupts);

        match self.lcd_status.status() {
            // horizontal blank
            LcdStatus::HBlank => {
                if self.clock >= HBLANK_PERIOD {
                    self.clock -= HBLANK_PERIOD;

                    self.lcd_y += 1;

                    let next_status = if self.lcd_y == 143 {
                        self.screen.swap_buffers();
                        LcdStatus::VBlank
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

                    self.lcd_y += 1;

                    // if 10 lines done since HBlank (i.e., 10 * VBLANK_PERIOD ticks elapsed)
                    if self.lcd_y > 153 {
                        self.lcd_y = 0;
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

                    self.screen.write_scanline();
                }
            }
        }
    }

    fn compare_ly_lyc(&mut self, interrupts: &mut Interrupts) {
        self.lcd_status
            .set_ly_lyc_equal(self.lcd_y == self.ly_compare);

        if self.lcd_status.ly_lyc_equal() {
            interrupts.flag(Interrupt::LcdStat, true);
        }
    }
}

#[derive(Clone, Copy)]
pub struct LcdControlRegister(pub u8);

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

#[derive(Clone, Copy)]
pub struct LcdStatusRegister(pub u8);

#[allow(dead_code)]
impl LcdStatusRegister {
    fn status(&self) -> LcdStatus {
        LcdStatus::from_u8(get_bits(self.0, 0, 2)).unwrap()
    }
    fn set_status(&mut self, status: LcdStatus) {
        self.0 = modify_bits(self.0, 0, 2, status as u8);
    }

    bit_accessors!(2, ly_lyc_equal, set_ly_lyc_equal);
}

#[derive(FromPrimitive)]
enum LcdStatus {
    HBlank,
    VBlank,
    SearchingOAM,
    TransferringDataToController,
}
