use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;

    let mut application = Application::default();
    event_loop.run_app(&mut application)?;

    Ok(())
}

#[derive(Default)]
struct Application {
    window: Option<Window>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes())
            .unwrap();

        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        _: WindowEvent,
    ) {
    }
}
