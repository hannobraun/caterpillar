use std::cmp;

use crossbeam_channel::{RecvError, TryRecvError};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::PhysicalSize,
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    keyboard::{Key, NamedKey},
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

    let event_loop = EventLoop::new()?;
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

    event_loop.run(move |event, event_loop_window_target| {
        // `desktop_threads` should always be `Some(...)`, unless we've called
        // `prepare_exit` in a previous loop iteration. I don't know if this can
        // ever happen, and that's going to depend on winit internals.
        //
        // In any case, *if* it can happen, we're already about to exit, so it
        // doesn't matter if we don't receive any more pixel operations.
        if let Some(DesktopThread { pixel_ops, .. }) = desktop_thread.as_ref() {
            loop {
                match pixel_ops.try_recv() {
                    Ok(pixel_op) => pixel_ops_buffer.push(pixel_op),
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => {
                        // This happens if the other end is dropped, for example
                        // when the application is shutting down.
                        prepare_exit(
                            &mut desktop_thread,
                            event_loop_window_target,
                        );
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
                prepare_exit(&mut desktop_thread, event_loop_window_target);
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key: Key::Named(NamedKey::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                prepare_exit(&mut desktop_thread, event_loop_window_target);
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                pixels.render().unwrap();
                window.request_redraw();
            }
            _ => {}
        }
    })?;

    Ok(())
}

fn prepare_exit(
    desktop_thread: &mut Option<DesktopThread>,
    event_loop_window_target: &EventLoopWindowTarget<()>,
) {
    // In principle, this function should just receive a `DesktopThread`, but
    // all its call sites are in the event handling closure, and it's not
    // possible to move out of a variable there. (Because it's not an `FnOnce`,
    // and hence the type system can't know it won't be called again.)
    //
    // This also means this function can get called multiple times, and only the
    // first call will join the `DesktopThread`. This might or might not ever
    // happen, depending on the implementation details of winit, and it doesn't
    // matter anyway. The first call will take care of the `DesktopThread`, and
    // any subsequent calls are just idempotent.
    if let Some(desktop_thread) = desktop_thread.take() {
        if let Err(err) = desktop_thread.quit() {
            eprintln!("{err:?}");
        }
    }

    event_loop_window_target.exit();
}

const WIDTH: u32 = 10;
const HEIGHT: u32 = 18;
