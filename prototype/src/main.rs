use std::sync::{
    mpsc::{self, RecvError, SendError},
    Arc,
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

fn main() -> anyhow::Result<()> {
    let (error_tx, error_rx) = mpsc::channel();

    let mut application = Application {
        resources: None,
        error: error_tx,
    };

    let event_loop = EventLoop::new()?;
    event_loop.run_app(&mut application)?;

    match error_rx.recv() {
        Ok(err) => return Err(err),
        Err(RecvError) => {
            // The other end has hung up. If it didn't send us an error before,
            // then all should be well.
        }
    }

    Ok(())
}

struct Application {
    resources: Option<ApplicationResources>,
    error: mpsc::Sender<anyhow::Error>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.resources = match ApplicationResources::init(event_loop) {
            Ok(resources) => Some(resources),
            Err(err) => {
                if let Err(SendError(_)) = self.error.send(err) {
                    // The other end has already hung up. Nothing we can do
                    // about it.
                };
                event_loop.exit();
                return;
            }
        };
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        _: WindowEvent,
    ) {
        let Some(resources) = self.resources.as_ref() else {
            return;
        };
        let _ = resources.window;
    }
}

struct ApplicationResources {
    window: Arc<Window>,
}

impl ApplicationResources {
    fn init(event_loop: &ActiveEventLoop) -> anyhow::Result<Self> {
        let window = {
            let window =
                event_loop.create_window(Window::default_attributes())?;
            Arc::new(window)
        };

        Ok(Self { window })
    }
}
