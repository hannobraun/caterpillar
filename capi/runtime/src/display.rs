use crate::tiles::{PIXELS_PER_AXIS, PIXELS_PER_TILE_AXIS, TILES_PER_AXIS};

pub struct Display {}

impl Display {
    pub fn set_tile(
        &mut self,
        x: usize,
        y: usize,
        value: u8,
        tiles: &mut [u8],
    ) {
        let index = || x.checked_add(y.checked_mul(TILES_PER_AXIS)?);
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
                        let num_channels = 4;

                        let frame_x = (tile_x * PIXELS_PER_TILE_AXIS
                            + offset_x)
                            * num_channels;
                        let frame_y = (tile_y * PIXELS_PER_TILE_AXIS
                            + offset_y)
                            * num_channels;

                        let i = frame_y * PIXELS_PER_AXIS + frame_x;
                        pixels[i..i + num_channels].copy_from_slice(&color);
                    }
                }
            }
        }
    }
}
