use super::tiles::{Tile, TILE_SIZE_BYTES};

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
const VRAM_SIZE: usize = (VRAM_END - VRAM_START + 1) as usize;

pub struct VideoRam {
    data: [u8; VRAM_SIZE],
}

impl VideoRam {
    pub fn new() -> Self {
        VideoRam {
            data: [0; VRAM_SIZE],
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        debug_assert!(addr >= VRAM_START);
        self.data[(addr - VRAM_START) as usize]
    }

    pub fn write8(&mut self, addr: u16, value: u8) {
        debug_assert!(addr >= VRAM_START);
        self.data[(addr - VRAM_START) as usize] = value;
    }

    // Read a tile at the given offset in the memory area 0x8000 to 0x87FF.
    fn read_tile_unsigned_offset(&self, offset: u8) -> Tile {
        let start = offset as usize * TILE_SIZE_BYTES;
        let range = start..(start + TILE_SIZE_BYTES);

        let mut bytes = [0; TILE_SIZE_BYTES];
        bytes.copy_from_slice(&self.data[range]);

        Tile::from_vram_data(bytes)
    }

    // Read a tile at the given offset in the memory area 0x8800 to 0x8FFF. This method may be used over
    // [`read_tile_unsigned_offset`] for drawing the background or window when LCD control bit 4 is set.
    fn read_tile_signed_offset(&self, offset: i8) {
        unimplemented!()
    }
}
