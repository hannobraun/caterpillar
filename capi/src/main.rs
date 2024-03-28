use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let event_loop = EventLoop::new()?;
    let window = Window::new(&event_loop)?;

    event_loop.run(|event, _| match event {
        Event::AboutToWait => {
            window.request_redraw();
        }
        Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } => {}
        _ => {}
    })?;

    Ok(())
}
