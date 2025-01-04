use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

fn main() -> anyhow::Result<()> {
    let mut application = Application { window: None };

    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut application)?;

    Ok(())
}

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
