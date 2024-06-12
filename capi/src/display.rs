use pixels::{Pixels, SurfaceTexture};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
    WebDisplayHandle, WebWindowHandle,
};

use crate::{
    effects::DisplayEffect,
    state::Input,
    tiles::{PIXELS_PER_AXIS, PIXELS_PER_TILE_AXIS, TILES_PER_AXIS},
};

pub struct Display {
    pixels: Pixels,
}

impl Display {
    pub async fn new() -> anyhow::Result<Self> {
        let pixels = {
            let size_u32: u32 = PIXELS_PER_AXIS
                .try_into()
                .expect("Expected `SIZE` to fit into `u32`");

            let surface_texture =
                SurfaceTexture::new(size_u32, size_u32, &CanvasWindow);
            Pixels::new_async(size_u32, size_u32, surface_texture).await?
        };

        Ok(Self { pixels })
    }

    pub fn handle_effect(
        &mut self,
        effect: DisplayEffect,
        input: &mut Input,
        tiles: &mut [u8],
    ) {
        match effect {
            DisplayEffect::SetTile { x, y, value } => {
                self.set_tile(x.into(), y.into(), value, tiles);
            }
            DisplayEffect::SubmitTiles { reply } => {
                // Once the process no longer runs in a separate task, this will
                // no longer be needed.
                reply.send(()).unwrap();
            }
            DisplayEffect::ReadInput { reply } => {
                let input = self.read_input(input);
                reply.send(input).unwrap();
            }
        }
    }

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

    pub fn read_input(&mut self, input: &mut Input) -> i8 {
        input.buffer.pop_front().unwrap_or(0).try_into().unwrap()
    }

    pub fn render(&mut self, tiles: &[u8]) {
        for tile_y in 0..TILES_PER_AXIS {
            for tile_x in 0..TILES_PER_AXIS {
                let i = tile_y * TILES_PER_AXIS + tile_x;
                let tile = tiles[i];

                let color = if tile == 0 {
                    [0, 0, 0, 0]
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
                        self.pixels.frame_mut()[i..i + num_channels]
                            .copy_from_slice(&color);
                    }
                }
            }
        }

        if let Err(err) = self.pixels.render() {
            eprintln!("Render error: {err}");
        }
    }
}

struct CanvasWindow;

unsafe impl HasRawDisplayHandle for CanvasWindow {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        RawDisplayHandle::Web(WebDisplayHandle::empty())
    }
}

unsafe impl HasRawWindowHandle for CanvasWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut window_handle = WebWindowHandle::empty();
        window_handle.id = 1;

        RawWindowHandle::Web(window_handle)
    }
}
