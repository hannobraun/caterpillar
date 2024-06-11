use pixels::{Pixels, SurfaceTexture};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
    WebDisplayHandle, WebWindowHandle,
};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

use crate::{
    effects::{DisplayEffect, TILES_PER_AXIS},
    ffi,
    runner::RunnerHandle,
    state::Input,
};

pub async fn run() -> anyhow::Result<()> {
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
    let window = event_loop.create_window({
        #[allow(unused_mut)]
        let mut window_attributes =
            Window::default_attributes().with_title("Caterpillar");

        // This crate is _only_ compiled to WebAssembly, but it seems that Cargo
        // and rust-analyzer know know how to handle workspaces with mixed
        // targets, or at least don't handle them well.
        //
        // This at least prevents errors from constantly showing up in the IDE.
        #[cfg(target_arch = "wasm32")]
        {
            use web_sys::wasm_bindgen::JsCast;
            use winit::platform::web::WindowAttributesExtWebSys;

            let canvas = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .get_element_by_id("capi")
                .unwrap()
                .dyn_into::<web_sys::HtmlCanvasElement>()
                .unwrap();

            window_attributes = window_attributes.with_canvas(Some(canvas));
        }

        window_attributes
    })?;

    let mut state = State { window };

    event_loop.run_app(&mut state)?;

    Ok(())
}

struct State {
    window: Window,
}

impl ApplicationHandler for State {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: winit::window::WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::RedrawRequested = event {
            let mut state = ffi::STATE.inner.lock().unwrap();
            let state = state.get_or_insert_with(Default::default);

            let Some(display) = state.display.as_mut() else {
                // Display has not been initialized yet.
                return;
            };

            display.render();
        }
    }

    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        // This is temporary, while winit is being replaced. Once we have full
        // control over what runs when, the low-level FFI code can just pass
        // this kind of state as an argument.
        let mut state = ffi::STATE.inner.lock().unwrap();
        let state = state.get_or_insert_with(Default::default);

        let Some(display) = state.display.as_mut() else {
            // Display has not been initialized yet.
            return;
        };

        display.handle_effects(&mut state.input, &mut state.runner);
        self.window.request_redraw();
    }
}

pub struct Display {
    tiles: [u8; NUM_TILES],
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

        Ok(Self {
            tiles: [0; NUM_TILES],
            pixels,
        })
    }

    pub fn handle_effects(
        &mut self,
        input: &mut Input,
        runner: &mut RunnerHandle,
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

                    self.tiles[index] = value;
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

    pub fn render(&mut self) {
        for tile_y in 0..TILES_PER_AXIS {
            for tile_x in 0..TILES_PER_AXIS {
                let i = tile_y * TILES_PER_AXIS + tile_x;
                let tile = self.tiles[i];

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

const PIXELS_PER_TILE_AXIS: usize = 8;
const PIXELS_PER_AXIS: usize = TILES_PER_AXIS * PIXELS_PER_TILE_AXIS;
const NUM_TILES: usize = TILES_PER_AXIS * TILES_PER_AXIS;
