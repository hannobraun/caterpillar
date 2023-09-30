use std::cmp;

use crossbeam_channel::Receiver;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

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
    let window = WindowBuilder::new()
        .with_title("Caterpillar")
        .with_inner_size(PhysicalSize::new(surface_width, surface_height))
        .with_resizable(false)
        .build(&event_loop)?;

    let surface_texture =
        SurfaceTexture::new(surface_width, surface_height, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    let mut queued_pixel_ops = vec![first_pixel_op];

    event_loop.run(move |event, _, control_flow| {
        queued_pixel_ops.extend(pixel_ops.try_iter());

        for PixelOp::Set([x, y]) in queued_pixel_ops.drain(..) {
            let x = cmp::max(0, cmp::min(x as usize, WIDTH as usize - 1));
            let y = cmp::max(0, cmp::min(y as usize, HEIGHT as usize - 1));

            let r = (y * WIDTH as usize + x) * 4;
            let g = r + 1;
            let b = r + 2;
            let a = r + 3;

            pixels.frame_mut()[r] = 255;
            pixels.frame_mut()[g] = 255;
            pixels.frame_mut()[b] = 255;
            pixels.frame_mut()[a] = 255;
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control_flow.set_exit();
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                control_flow.set_exit();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                pixels.render().unwrap();
            }
            _ => {}
        }
    })
}

const WIDTH: u32 = 10;
const HEIGHT: u32 = 18;
