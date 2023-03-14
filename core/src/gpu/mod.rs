pub mod oam;
mod palettes;
pub mod vram;

use oam::SpriteAttributeTable;
use palettes::*;
use vram::{VideoRam, TILE_WIDTH};

use crate::bits::{bit_accessors, get_bits, modify_bits};
use crate::cycles::TCycles;
use crate::interrupts::{Interrupt, Interrupts};
use crate::screen::{Screen, SCREEN_WIDTH};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use self::oam::SPRITE_COUNT;

const HBLANK_PERIOD: TCycles = TCycles(204);
const VBLANK_PERIOD: TCycles = TCycles(456); // single line
const SEARCHING_OAM_PERIOD: TCycles = TCycles(80);
const TRANSFERRING_DATA_PERIOD: TCycles = TCycles(172);

pub struct Gpu {
    // The screen to which the GPU will draw.
    pub screen: Screen,
    // Video memory (contains tile map, tile data, etc.)
    pub vram: VideoRam,
    // Object attribute memory / sprite attribute table (contains sprite positions and attributes).
    pub oam: SpriteAttributeTable,
    // 0xFF40 - LCD control register.
    pub lcd_control: LcdControlRegister,
    // 0xFF41 - LCD status register.
    pub lcd_status: LcdStatusRegister,
    // 0xFF42 - Viewport Y (specify the visible area of the background).
    pub viewport_y: u8,
    // 0xFF43 - Viewport X (specify the visible area of the background).
    pub viewport_x: u8,
    // 0xFF44 - LCD Y (current scanline being drawn, from 0 to 153).
    pub lcd_y: u8,
    // 0xFF45 - LY compare (this value is compared with LCD Y and an interrupt is triggered when they're equal).
    pub ly_compare: u8,
    // 0xFF47 - Background colour palette.
    pub bg_palette_data: Palette,
    // 0xFF48 - First of two object (sprite) colour palettes.
    pub obj_palette_0_data: Palette,
    // 0xFF49 - Second of two object (sprite) colour palettes.
    pub obj_palette_1_data: Palette,
    // 0xFF4A - Window Y (position of the window).
    pub window_y: u8,
    // 0xFF4B - Window X (position of the window).
    pub window_x: u8,
    // Counter used to time transition between rendering states.
    clock: TCycles,
}

impl Gpu {
    pub fn new() -> Self {
        Gpu {
            screen: Screen::new(),
            vram: VideoRam::new(),
            oam: SpriteAttributeTable::new(),
            lcd_control: LcdControlRegister(0x91),
            lcd_status: LcdStatusRegister(0x81),
            viewport_y: 0,
            viewport_x: 0,
            lcd_y: 0x91,
            ly_compare: 0,
            bg_palette_data: Palette(0xFC),
            obj_palette_0_data: Palette(0),
            obj_palette_1_data: Palette(0),
            window_y: 0,
            window_x: 0,
            clock: TCycles(0),
        }
    }

    // Handle the transitions between rendering states and draw to the screen appropriately.
    pub fn update(&mut self, interrupts: &mut Interrupts, cycles: TCycles) {
        self.clock += cycles;

        self.compare_ly_lyc(interrupts);

        match self.lcd_status.status() {
            // horizontal blank
            LcdStatus::HBlank => {
                if self.clock >= HBLANK_PERIOD {
                    self.clock -= HBLANK_PERIOD;
                    let next = self.hblank(interrupts);
                    self.lcd_status.set_status(next);
                }
            }

            // vertical blank
            LcdStatus::VBlank => {
                if self.clock >= VBLANK_PERIOD {
                    self.clock -= VBLANK_PERIOD;
                    let next = self.vblank();
                    self.lcd_status.set_status(next);
                }
            }

            // scanline (accessing OAM)
            LcdStatus::SearchingOAM => {
                if self.clock >= SEARCHING_OAM_PERIOD {
                    self.clock -= SEARCHING_OAM_PERIOD;
                    let next = self.searching_oam();
                    self.lcd_status.set_status(next);
                }
            }

            // scanline (accessing VRAM)
            LcdStatus::TransferringData => {
                if self.clock >= TRANSFERRING_DATA_PERIOD {
                    self.clock -= TRANSFERRING_DATA_PERIOD;
                    let next = self.transferring_data();
                    self.lcd_status.set_status(next);
                }
            }
        }
    }

    // Called after a scanline has been drawn. Increments the LCD Y position and, if we've reached the button of the
    // screen, will flag the VBlank interrupt and transition to the VBlank state. If however we are not yet at the
    // bottom of the screen, then we will move on to the next scanline by transitioning to the 'searching OAM' state.
    fn hblank(&mut self, interrupts: &mut Interrupts) -> LcdStatus {
        self.lcd_y += 1;

        if self.lcd_y == 143 {
            interrupts.flag(Interrupt::VBlank, true);
            LcdStatus::VBlank
        } else {
            LcdStatus::SearchingOAM
        }
    }

    // Called after all scanlines have been drawn. Will remain in this state and increment LCD Y position until the
    // LCD Y position reaches 10 below the height of the screen, at which point LCD Y is reset to 0 and we transition
    // to the 'searching OAM' state.
    fn vblank(&mut self) -> LcdStatus {
        self.lcd_y += 1;

        // if 10 lines done since final HBlank (i.e., 10 * VBLANK_PERIOD ticks elapsed)
        if self.lcd_y > 153 {
            self.lcd_y = 0;
            return LcdStatus::SearchingOAM;
        }

        LcdStatus::VBlank
    }

    // TODO
    fn searching_oam(&mut self) -> LcdStatus {
        LcdStatus::TransferringData
    }

    // In this state we handle drawing a scanline to the screen before then transitioning to the HBlank state.
    fn transferring_data(&mut self) -> LcdStatus {
        self.draw_scanline();
        LcdStatus::HBlank
    }

    fn compare_ly_lyc(&mut self, interrupts: &mut Interrupts) {
        self.lcd_status
            .set_ly_lyc_equal(self.lcd_y == self.ly_compare);

        if self.lcd_status.ly_lyc_equal() {
            interrupts.flag(Interrupt::LcdStat, true);
        }
    }

    fn draw_scanline(&mut self) {
        self.draw_background_scanline();
        self.draw_sprites_scanline();
    }

    fn draw_background_scanline(&mut self) {
        for x in (0..SCREEN_WIDTH).step_by(TILE_WIDTH) {
            let map_x = (x / TILE_WIDTH) as u8;
            let map_y = self.lcd_y / TILE_WIDTH as u8;
            let tile_index = self.vram.read_tile_index_from_map_9800(map_x, map_y);

            let line_number = self.lcd_y % TILE_WIDTH as u8;
            let colour_ids = self
                .vram
                .read_tile_line_unsigned_index(tile_index, line_number);

            for (i, colour_id) in colour_ids.into_iter().enumerate() {
                let colour = self.bg_palette_data.colour_for_id(colour_id);
                self.screen.set((x + i) as u8, self.lcd_y, colour);
            }
        }
    }

    fn draw_sprites_scanline(&mut self) {
        let mut sprites_drawn = 0;

        for index in 0..SPRITE_COUNT {
            let sprite = self.oam.read_sprite(index);

            let scanline_in_sprite_bounds =
                (sprite.y..(sprite.y.saturating_add(TILE_WIDTH as u8))).contains(&self.lcd_y);

            if scanline_in_sprite_bounds {
                // TODO: Handle flipping, priority, etc.

                let colour_ids = self
                    .vram
                    .read_tile_line_unsigned_index(sprite.tile_index, self.lcd_y - sprite.y);

                let palette = if sprite.use_palette_1 {
                    &self.obj_palette_1_data
                } else {
                    &self.obj_palette_0_data
                };

                for (i, colour_id) in colour_ids.into_iter().enumerate() {
                    // the X coordinate of sprite is stored off by 8 pixels and the y coordinate by 16 pixels so need
                    // to check if we're in bounds to prevent integer underflow when subtracting
                    // also, a colour ID of 0 indicates transparency
                    if sprite.x >= 8 && colour_id != 0 {
                        let colour = palette.colour_for_id(colour_id);
                        self.screen
                            .set(sprite.x - 8 + i as u8, self.lcd_y - 16, colour);
                    }
                }

                sprites_drawn += 1;
            }

            if sprites_drawn >= 10 {
                break;
            }
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

#[derive(Debug, FromPrimitive)]
enum LcdStatus {
    HBlank,
    VBlank,
    SearchingOAM,
    TransferringData,
}
