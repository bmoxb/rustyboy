use crate::bits::get_bit;
use crate::gpu::palettes::BackgroundPalette;

pub const VRAM_START: u16 = 0x8000;
pub const VRAM_END: u16 = 0x9FFF;
const VRAM_SIZE: usize = (VRAM_END - VRAM_START + 1) as usize;

pub const TILE_WIDTH: usize = 8;
const TILE_SIZE_BYTES: usize = 16;

const TILE_MAP_WIDTH: usize = 32;

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

    // Read a tile at the given index in the memory area 0x8000 to 0x87FF.
    pub fn read_tile_line_unsigned_index(&self, offset: u8, line: u8) -> [u8; TILE_WIDTH] {
        debug_assert!((line as usize) < TILE_WIDTH);

        // `line * 2` below is because there are two bytes per line
        let addr = 0x8000 + (offset as u16 * TILE_SIZE_BYTES as u16) + (line as u16 * 2);

        self.read_tile_line(addr)
    }

    // Read a tile at the given index in the memory area 0x8800 to 0x8FFF. This method may be used over
    // [`read_tile_unsigned_index`] for drawing the background or window when LCD control bit 4 is set.
    pub fn read_tile_line_signed_index(&self, _offset: i8, line: u8) -> [u8; TILE_WIDTH] {
        debug_assert!((line as usize) < TILE_WIDTH);
        unimplemented!() // TODO
    }

    pub fn read_tile_index_from_map_9800(&self, x: u8, y: u8) -> u8 {
        self.read_tile_index(0x9800, x, y)
    }

    pub fn read_tile_index_from_map_9C00(&self, x: u8, y: u8) -> u8 {
        self.read_tile_index(0x9C00, x, y)
    }

    fn read_tile_line(&self, addr: u16) -> [u8; TILE_WIDTH] {
        let (top, bottom) = (self.read8(addr), self.read8(addr + 1));
        parse_tile_line_from_byte_pair(top, bottom)
    }

    fn read_tile_index(&self, offset: u16, x: u8, y: u8) -> u8 {
        debug_assert!((x as usize) < TILE_MAP_WIDTH && (y as usize) < TILE_MAP_WIDTH);
        self.read8(offset + (y as u16 * TILE_MAP_WIDTH as u16) + x as u16)
    }
}

fn parse_tile_line_from_byte_pair(top: u8, bottom: u8) -> [u8; TILE_WIDTH] {
    let mut colour_ids = [0; TILE_WIDTH];

    for i in 0..TILE_WIDTH as u8 {
        let high = get_bit(bottom, i);
        let low = get_bit(top, i);
        let colour_id = ((high as u8) << 1) + low as u8;

        let index = TILE_WIDTH - i as usize - 1;
        colour_ids[index] = colour_id;
    }

    colour_ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tile_data() {
        let bytes = [
            0x7C, 0x7C, 0x00, 0xC6, 0xC6, 0x00, 0x00, 0xFE, 0xC6, 0xC6, 0x00, 0xC6, 0xC6, 0x00,
            0x00, 0x00,
        ];
        let colour_ids = [
            0, 3, 3, 3, 3, 3, 0, 0, // 0x7C, 0x7C
            2, 2, 0, 0, 0, 2, 2, 0, // 0x00, 0xC6
            1, 1, 0, 0, 0, 1, 1, 0, // 0xC6, 0x00
            2, 2, 2, 2, 2, 2, 2, 0, // 0x00, 0xFE
            3, 3, 0, 0, 0, 3, 3, 0, // 0xC6, 0xC6
            2, 2, 0, 0, 0, 2, 2, 0, // 0x00, 0xC6
            1, 1, 0, 0, 0, 1, 1, 0, // 0xC6, 0x00
            0, 0, 0, 0, 0, 0, 0, 0, // 0x00, 0x00
        ];

        for i in 0..8 {
            let (top, bottom) = (bytes[i * 2], bytes[i * 2 + 1]);

            let line = parse_tile_line_from_byte_pair(top, bottom);
            let expected_line = &colour_ids[(i * 8)..((i + 1) * 8)];

            assert_eq!(line, expected_line);
        }
    }
}
