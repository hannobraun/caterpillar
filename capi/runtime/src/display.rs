use crate::tiles::{NUM_CHANNELS, PIXELS_PER_AXIS, PIXELS_PER_TILE_AXIS};

pub fn set_tile(tile_x: usize, tile_y: usize, value: u8, tiles: &mut [u8]) {
    let color = if value == 0 {
        [0, 0, 0, 255]
    } else {
        [255, 255, 255, 255]
    };

    for offset_y in 0..PIXELS_PER_TILE_AXIS {
        for offset_x in 0..PIXELS_PER_TILE_AXIS {
            let pixel_x =
                (tile_x * PIXELS_PER_TILE_AXIS + offset_x) * NUM_CHANNELS;
            let pixel_y =
                (tile_y * PIXELS_PER_TILE_AXIS + offset_y) * NUM_CHANNELS;

            let i = pixel_y * PIXELS_PER_AXIS + pixel_x;
            tiles[i..i + NUM_CHANNELS].copy_from_slice(&color);
        }
    }
}
