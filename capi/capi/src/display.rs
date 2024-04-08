use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

use crate::capi::Program;

pub fn run(mut program: Program) -> anyhow::Result<()> {
    const TILES_PER_AXIS: usize = 32;
    const PIXELS_PER_TILE_AXIS: usize = 8;

    // I don't like the `as`, but I can't use `try_into` in a const context.
    // Given this is a screen resolution, this is unlikely to ever be a problem.
    const SIZE: usize = TILES_PER_AXIS * PIXELS_PER_TILE_AXIS;
    let size_u32: u32 =
        SIZE.try_into().expect("Expected `SIZE` to fit into `u32`");

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("Caterpillar")
        .build(&event_loop)?;

    let surface_texture = SurfaceTexture::new(size_u32, size_u32, &window);
    let mut pixels = Pixels::new(size_u32, size_u32, surface_texture)?;

    event_loop.run(|event, event_loop_window_target| match event {
        Event::AboutToWait => {
            program.run(SIZE, SIZE, pixels.frame_mut());
            window.request_redraw();
        }
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            event_loop_window_target.exit();
        }
        Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    event:
                        KeyEvent {
                            physical_key: PhysicalKey::Code(KeyCode::Escape),
                            ..
                        },
                    ..
                },
            ..
        } => {
            event_loop_window_target.exit();
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            if let Err(err) = pixels.render() {
                eprintln!("Render error: {err}");
            }
        }
        _ => {}
    })?;

    Ok(())
}
