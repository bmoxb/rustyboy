use crate::bits::get_bit;

const TILE_WIDTH: usize = 8;
pub const TILE_SIZE_BYTES: usize = 16;

pub struct Tile {
    colour_ids: [u8; TILE_WIDTH * TILE_WIDTH],
}

impl Tile {
    pub fn from_vram_data(bytes: [u8; TILE_SIZE_BYTES]) -> Self {
        let mut colour_ids = [0; TILE_WIDTH * TILE_WIDTH];

        for (row, chunk) in bytes.chunks(2).enumerate() {
            if let &[top, bottom] = chunk {
                for col in 0..TILE_WIDTH {
                    let high = get_bit(bottom, col as u8);
                    let low = get_bit(top, col as u8);
                    let colour_id = ((high as u8) << 1) + low as u8;

                    let index = row * TILE_WIDTH + (TILE_WIDTH - col - 1);
                    colour_ids[index] = colour_id;
                }
            }
        }

        Tile { colour_ids }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_from_vram_data() {
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

        let t = Tile::from_vram_data(bytes);
        assert_eq!(t.colour_ids, colour_ids);
    }
}
