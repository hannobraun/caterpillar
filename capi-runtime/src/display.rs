use std::collections::VecDeque;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::{
    effects::{DisplayEffect, TILES_PER_AXIS},
    runner::RunnerHandle,
};

pub async fn run(runner: RunnerHandle) -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    // Winit's new approach won't work for us in the browser, as `Pixels`
    // doesn't have its blocking constructor there, and we need to use an async
    // one. But winit's new `ApplicationHandler` isn't async-aware, and we can't
    // block on a future in the browser, as far as I understand.
    //
    // So we're back to the old and deprecated way. It won't matter much, as I
    // think winit is a bit too heavyweight anyway, for what I'm trying to do
    // here, and I plan to phase it out.
    #[allow(deprecated)]
    let window = event_loop.create_window(
        Window::default_attributes().with_title("Caterpillar"),
    )?;

    let pixels = {
        let size_u32: u32 = PIXELS_PER_AXIS
            .try_into()
            .expect("Expected `SIZE` to fit into `u32`");

        let surface_texture = SurfaceTexture::new(size_u32, size_u32, &window);
        Pixels::new_async(size_u32, size_u32, surface_texture).await?
    };

    let mut state = State {
        runner,
        mem: [0; MEM_SIZE],
        window,
        pixels,
        input: VecDeque::new(),
    };

    event_loop.run_app(&mut state)?;

    Ok(())
}

struct State {
    runner: RunnerHandle,
    mem: [u8; MEM_SIZE],
    window: Window,
    pixels: Pixels,
    input: VecDeque<i8>,
}

impl ApplicationHandler for State {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key:
                            PhysicalKey::Code(KeyCode::ArrowUp | KeyCode::KeyW),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.input.push_back(1);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key:
                            PhysicalKey::Code(KeyCode::ArrowLeft | KeyCode::KeyA),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.input.push_back(2);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key:
                            PhysicalKey::Code(KeyCode::ArrowDown | KeyCode::KeyS),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.input.push_back(3);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key:
                            PhysicalKey::Code(KeyCode::ArrowRight | KeyCode::KeyD),
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => {
                self.input.push_back(4);
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = self.pixels.render() {
                    eprintln!("Render error: {err}");
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        for effect in self.runner.effects() {
            match effect {
                DisplayEffect::SetTile { x, y, value } => {
                    let x_usize: usize = x.into();
                    let y_usize: usize = y.into();

                    let index = || {
                        x_usize
                            .checked_add(y_usize.checked_mul(TILES_PER_AXIS)?)
                    };
                    let index = index().unwrap();

                    self.mem[index] = value;
                }
                DisplayEffect::SubmitTiles { reply } => {
                    reply.send(()).unwrap();
                }
                DisplayEffect::ReadInput { reply } => {
                    let input = self.input.pop_front().unwrap_or(0);
                    reply.send(input).unwrap();
                }
            }
        }

        for tile_y in 0..TILES_PER_AXIS {
            for tile_x in 0..TILES_PER_AXIS {
                let i = tile_y * TILES_PER_AXIS + tile_x;
                let tile = self.mem[i];

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

        self.window.request_redraw();
    }
}

const PIXELS_PER_TILE_AXIS: usize = 8;
const PIXELS_PER_AXIS: usize = TILES_PER_AXIS * PIXELS_PER_TILE_AXIS;
const MEM_SIZE: usize = TILES_PER_AXIS * TILES_PER_AXIS;
