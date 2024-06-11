use pixels::{Pixels, SurfaceTexture};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
    WebDisplayHandle, WebWindowHandle,
};

use crate::{
    effects::{DisplayEffect, TILES_PER_AXIS},
    runner::RunnerHandle,
    state::Input,
    tiles::{PIXELS_PER_AXIS, PIXELS_PER_TILE_AXIS},
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

    pub fn handle_effects(
        &mut self,
        input: &mut Input,
        runner: &mut RunnerHandle,
        tiles: &mut [u8],
    ) {
        for effect in runner.effects() {
            match effect {
                DisplayEffect::SetTile { x, y, value } => {
                    let x_usize: usize = x.into();
                    let y_usize: usize = y.into();

                    let index = || {
                        x_usize
                            .checked_add(y_usize.checked_mul(TILES_PER_AXIS)?)
                    };
                    let index = index().unwrap();

                    tiles[index] = value;
                }
                DisplayEffect::SubmitTiles { reply } => {
                    reply.send(()).unwrap();
                }
                DisplayEffect::ReadInput { reply } => {
                    let input = input.buffer.pop_front().unwrap_or(0);
                    reply.send(input.try_into().unwrap()).unwrap();
                }
            }
        }
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
