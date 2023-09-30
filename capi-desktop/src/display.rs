use std::cmp;

use crossbeam_channel::Receiver;
use pixels::{Pixels, SurfaceTexture};
use winit::{event_loop::EventLoop, window::Window};

use crate::platform::PixelOp;

pub fn start(pixel_ops: Receiver<PixelOp>) -> anyhow::Result<()> {
    // Block until the first pixel op is sent.
    let first_pixel_op = pixel_ops.recv()?;

    // If a pixel op has been sent, initialize the display and start handling
    // pixel ops for real.

    let factor = 40;

    let buffer_to_surface = |size| size * factor;

    let surface_width = buffer_to_surface(WIDTH);
    let surface_height = buffer_to_surface(HEIGHT);

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop)?;

    let surface_texture =
        SurfaceTexture::new(surface_width, surface_height, &window);
    let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    let mut display = Display {
        _event_loop: event_loop,
        pixels,
    };
    let pixel_ops = [first_pixel_op].into_iter().chain(pixel_ops.iter());

    for PixelOp::Set(position) in pixel_ops {
        let [x, y] = position.map(|value| {
            let min = 0;
            let max = cmp::max(WIDTH, HEIGHT).into();

            value.max(min).min(max) as usize
        });

        let r = y * WIDTH as usize + x;
        let g = r + 1;
        let b = r + 2;
        let a = r + 3;

        display.pixels.frame_mut()[r] = 255;
        display.pixels.frame_mut()[g] = 255;
        display.pixels.frame_mut()[b] = 255;
        display.pixels.frame_mut()[a] = 255;

        display.pixels.render()?;
    }

    Ok(())
}

pub struct Display {
    _event_loop: EventLoop<()>,
    pixels: Pixels,
}

const WIDTH: u32 = 10;
const HEIGHT: u32 = 18;
