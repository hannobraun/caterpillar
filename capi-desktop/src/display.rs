use std::cmp;

use crossbeam_channel::Receiver;
use pixels::{Pixels, SurfaceTexture};
use winit::{event::Event, event_loop::EventLoop, window::Window};

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
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    let mut queued_pixel_ops = vec![first_pixel_op];

    event_loop.run(move |event, _, _| {
        queued_pixel_ops.extend(pixel_ops.try_iter());

        for PixelOp::Set(position) in queued_pixel_ops.drain(..) {
            let [x, y] = position.map(|value| {
                let min = 0;
                let max = cmp::max(WIDTH, HEIGHT).into();

                value.max(min).min(max) as usize
            });

            let r = y * WIDTH as usize + x;
            let g = r + 1;
            let b = r + 2;
            let a = r + 3;

            pixels.frame_mut()[r] = 255;
            pixels.frame_mut()[g] = 255;
            pixels.frame_mut()[b] = 255;
            pixels.frame_mut()[a] = 255;
        }

        #[allow(clippy::single_match)]
        match event {
            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            }
            _ => {}
        }
    })
}

const WIDTH: u32 = 10;
const HEIGHT: u32 = 18;
