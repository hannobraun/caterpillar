use capi_runtime::{Program, ProgramState, Value};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::server::{EventsRx, UpdatesTx};

pub fn run(
    program: Program,
    events: EventsRx,
    updates: UpdatesTx,
) -> anyhow::Result<()> {
    let size_u32: u32 =
        SIZE.try_into().expect("Expected `SIZE` to fit into `u32`");

    let event_loop = EventLoop::new()?;
    #[allow(deprecated)] // only for the transition to winit 0.30
    let window = event_loop.create_window(
        Window::default_attributes().with_title("Caterpillar"),
    )?;

    let surface_texture = SurfaceTexture::new(size_u32, size_u32, &window);
    let pixels = Pixels::new(size_u32, size_u32, surface_texture)?;

    let mut state = State {
        program,
        events,
        updates,
        mem: [0; MEM_SIZE],
        window,
        pixels,
    };

    #[allow(deprecated)] // only for the transition to winit 0.30
    event_loop.run(|event, event_loop_window_target| match event {
        Event::AboutToWait => {
            while let Ok(event) = state.events.try_recv() {
                state.program.apply_debug_event(event);
                state.updates.send(state.program.clone()).unwrap();
            }

            if let ProgramState::Finished = state.program.state {
                state
                    .program
                    .push([Value(TILES_PER_AXIS.try_into().unwrap()); 2]);
                state.program.reset();
            }

            let previous_state = state.program.state.clone();

            loop {
                if let ProgramState::Error { .. } = state.program.state {
                    // If there's an error, never run the program again. As of
                    // this writing, that's it, and for now that's fine.
                    //
                    // Eventually, it would be great if we could get out of this
                    // by resetting the program, or rewinding the builtin that
                    // caused the error.
                    break;
                }

                match state.program.step(&mut state.mem) {
                    ProgramState::Running => {}
                    ProgramState::Paused { .. } => {
                        break;
                    }
                    ProgramState::Finished => {
                        assert_eq!(
                            state.program.evaluator.data_stack.num_values(),
                            0
                        );
                        break;
                    }
                    ProgramState::Error { .. } => {
                        break;
                    }
                }
            }

            if state.program.state != previous_state {
                state.updates.send(state.program.clone()).unwrap();
            }

            for tile_y in 0..TILES_PER_AXIS {
                for tile_x in 0..TILES_PER_AXIS {
                    let i = TILES_OFFSET + tile_y * TILES_PER_AXIS + tile_x;
                    let tile = state.mem[i];

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

                            let i = frame_y * SIZE + frame_x;
                            state.pixels.frame_mut()[i..i + num_channels]
                                .copy_from_slice(&color);
                        }
                    }
                }
            }

            state.window.request_redraw();
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
            if let Err(err) = state.pixels.render() {
                eprintln!("Render error: {err}");
            }
        }
        _ => {}
    })?;

    Ok(())
}

const TILES_PER_AXIS: usize = 32;
const PIXELS_PER_TILE_AXIS: usize = 8;

const SIZE: usize = TILES_PER_AXIS * PIXELS_PER_TILE_AXIS;

const TILES_OFFSET: usize = 256;

const MEM_SIZE: usize = TILES_OFFSET + TILES_PER_AXIS * TILES_PER_AXIS;

struct State {
    program: Program,
    events: EventsRx,
    updates: UpdatesTx,
    mem: [u8; MEM_SIZE],
    window: Window,
    pixels: Pixels,
}
