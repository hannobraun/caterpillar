use std::cmp;

use crossbeam_channel::{RecvError, TryRecvError};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{platform::PixelOp, DesktopThread};

/// Start the display
///
/// This method blocks until the first pixel op is received, then initializes
/// the display and starts the normal event handling. Once it reaches that point
/// it will never return.
///
/// Unfortunately, due to that weird "block, maybe do nothing, or initialize"
/// behavior, it's not possible to express this "never return" thing in the
/// method signature, as the method might return (both `Ok` or `Err`) before
/// initialization is done.
///
/// This is probably an argument for representing the display as a value within
/// Caterpillar, which will require some extensions to the value system.
pub fn start(desktop_thread: DesktopThread) -> anyhow::Result<()> {
    // Block until the first pixel op is sent.
    let first_pixel_op = match desktop_thread.pixel_ops.recv() {
        Ok(pixel_op) => pixel_op,
        Err(RecvError) => {
            // This happens if the other end is disconnected, for example
            // when the application shuts down. If this happens here, then
            // the Caterpillar program never needed the services of this
            // code, and we can just quietly quit.
            desktop_thread.join()?;
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

    let mut desktop_thread = Some(desktop_thread);
    let mut pixel_ops_buffer = vec![first_pixel_op];

    event_loop.run(move |event, _, control_flow| {
        if let Some(DesktopThread { pixel_ops, .. }) = desktop_thread.as_ref() {
            loop {
                match pixel_ops.try_recv() {
                    Ok(pixel_op) => pixel_ops_buffer.push(pixel_op),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        // This happens if the other end is dropped, for example
                        // when the application is shutting down.
                        prepare_exit(&mut desktop_thread, control_flow);
                        return;
                    }
                }
            }
        }

        for pixel_op in pixel_ops_buffer.drain(..) {
            let ([x, y], value) = match pixel_op {
                PixelOp::Clear(pos) => (pos, 0),
                PixelOp::Set(pos) => (pos, 255),
            };

            let clamp = |value, max| {
                cmp::min(max as usize - 1, cmp::max(0, value) as usize)
            };

            let x = clamp(x, WIDTH);
            let y = clamp(y, HEIGHT);

            let r = (y * WIDTH as usize + x) * 4;
            let g = r + 1;
            let b = r + 2;
            let a = r + 3;

            pixels.frame_mut()[r] = value;
            pixels.frame_mut()[g] = value;
            pixels.frame_mut()[b] = value;
            pixels.frame_mut()[a] = value;
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                prepare_exit(&mut desktop_thread, control_flow);
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
                prepare_exit(&mut desktop_thread, control_flow);
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

fn prepare_exit(
    desktop_thread: &mut Option<DesktopThread>,
    control_flow: &mut ControlFlow,
) {
    if let Some(desktop_thread) = desktop_thread.take() {
        match desktop_thread.join() {
            Ok(()) => {}
            Err(err) => {
                eprintln!("{err:?}");
            }
        }
    }
    control_flow.set_exit();
}

const WIDTH: u32 = 10;
const HEIGHT: u32 = 18;
