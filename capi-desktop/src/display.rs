use std::cmp;

use crossbeam_channel::{Receiver, RecvError, TryRecvError};
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
    let first_pixel_op = match pixel_ops.recv() {
        Ok(pixel_op) => pixel_op,
        Err(RecvError) => {
            // This happens if the other end is disconnected, for example
            // when the application shuts down. If this happens here, then
            // the Caterpillar program never needed the services of this
            // code, and we can just quietly quit.
            return Ok(());
        }
    };

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

    let mut pixel_ops_buffer = vec![first_pixel_op];

    event_loop.run(move |event, _, control_flow| {
        loop {
            match pixel_ops.try_recv() {
                Ok(pixel_op) => pixel_ops_buffer.push(pixel_op),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    // This happens if the other end is dropped, for example
                    // when the application is shutting down.
                    control_flow.set_exit();
                    return;
                }
            }
        }

        for pixel_op in pixel_ops_buffer.drain(..) {
            let PixelOp::Set([x, y]) = pixel_op;

            let clamp = |value, max| {
                cmp::min(max as usize - 1, cmp::max(0, value) as usize)
            };

            let x = clamp(x, WIDTH);
            let y = clamp(y, HEIGHT);

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
