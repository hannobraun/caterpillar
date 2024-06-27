use crate::tiles::{
    NUM_CHANNELS, PIXELS_PER_AXIS, PIXELS_PER_TILE_AXIS, TILES_PER_AXIS,
};

pub fn set_tile(tile_x: usize, tile_y: usize, value: u8, tiles: &mut [u8]) {
    let index = || tile_x.checked_add(tile_y.checked_mul(TILES_PER_AXIS)?);
    let index = index().unwrap();

    tiles[index] = value;
}

pub fn render(tiles: &[u8], pixels: &mut [u8]) {
    for tile_y in 0..TILES_PER_AXIS {
        for tile_x in 0..TILES_PER_AXIS {
            let i = tile_y * TILES_PER_AXIS + tile_x;
            let tile = tiles[i];

            let color = if tile == 0 {
                [0, 0, 0, 255]
            } else {
                [255, 255, 255, 255]
            };

            for offset_y in 0..PIXELS_PER_TILE_AXIS {
                for offset_x in 0..PIXELS_PER_TILE_AXIS {
                    let frame_x = (tile_x * PIXELS_PER_TILE_AXIS + offset_x)
                        * NUM_CHANNELS;
                    let frame_y = (tile_y * PIXELS_PER_TILE_AXIS + offset_y)
                        * NUM_CHANNELS;

                    let i = frame_y * PIXELS_PER_AXIS + frame_x;
                    pixels[i..i + NUM_CHANNELS].copy_from_slice(&color);
                }
            }
        }
    }
}
