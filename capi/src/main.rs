use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 320;

    let event_loop = EventLoop::new()?;
    let window = Window::new(&event_loop)?;

    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    event_loop.run(|event, _| match event {
        Event::AboutToWait => {
            window.request_redraw();
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {
            pixels.render().unwrap();
        }
        _ => {}
    })?;

    Ok(())
}
