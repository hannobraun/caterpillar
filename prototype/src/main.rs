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
        window: None,
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
    window: Option<ApplicationResources>,
    error: mpsc::Sender<anyhow::Error>,
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = match init(event_loop) {
            Ok(window) => window,
            Err(err) => {
                if let Err(SendError(_)) = self.error.send(err) {
                    // The other end has hung up. Nothing else we can do
                    // with this error now.
                };
                event_loop.exit();
                return;
            }
        };

        self.window = Some(ApplicationResources { window });
    }

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        _: WindowEvent,
    ) {
        let Some(resources) = self.window.as_ref() else {
            return;
        };
        let _ = resources.window;
    }
}

struct ApplicationResources {
    window: Arc<Window>,
}

fn init(event_loop: &ActiveEventLoop) -> anyhow::Result<Arc<Window>> {
    let window = {
        let window = event_loop.create_window(Window::default_attributes())?;
        Arc::new(window)
    };

    Ok(window)
}
