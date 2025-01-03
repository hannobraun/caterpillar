use std::sync::{
    mpsc::{self, SendError, TryRecvError},
    Arc,
};

use anyhow::anyhow;
use pollster::FutureExt;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::{Key, NamedKey},
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

    match error_rx.try_recv() {
        Ok(err) => return Err(err),
        Err(TryRecvError::Empty) => {
            // There's no error in the channel. All should be well.
        }
        Err(TryRecvError::Disconnected) => {
            unreachable!(
                "Error channel can't disconnect. The sender lives on the local \
                stack."
            );
        }
    }

    Ok(())
}

struct Application {
    resources: Option<ApplicationResources>,
    error: mpsc::Sender<anyhow::Error>,
}

impl Application {
    fn handle_error(&self, err: anyhow::Error, event_loop: &ActiveEventLoop) {
        if let Err(SendError(err)) = self.error.send(err) {
            // The other end has already hung up. Nothing we can do
            // about it.
            println!(
                "Error while initializing application resources:\n\
                {err:?}\n\
                \n\
                Failed to report this error properly, as the main thread isn't
                listening anymore."
            );
        };
        event_loop.exit();
    }
}

impl ApplicationHandler for Application {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.resources = match ApplicationResources::new(event_loop) {
            Ok(resources) => Some(resources),
            Err(err) => {
                self.handle_error(err, event_loop);
                return;
            }
        };
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let Some(resources) = self.resources.as_ref() else {
            return;
        };
        let _ = resources.window;

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Err(err) = resources.renderer.render() {
                    self.handle_error(err, event_loop);

                    // I want to have this explicit return here, to make sure
                    // this stays working as the code here shifts.
                    #[allow(clippy::needless_return)]
                    return;
                }
            }
            _ => {}
        }
    }
}

struct ApplicationResources {
    window: Arc<Window>,
    renderer: Renderer,
}

impl ApplicationResources {
    fn new(event_loop: &ActiveEventLoop) -> anyhow::Result<Self> {
        let window = {
            let window =
                event_loop.create_window(Window::default_attributes())?;
            Arc::new(window)
        };

        let renderer = Renderer::new(&window).block_on()?;

        Ok(Self { window, renderer })
    }
}

struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Renderer {
    async fn new(window: &Arc<Window>) -> anyhow::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::default());
        let surface = instance.create_surface(window.clone())?;

        let Some(adapter) = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
        else {
            return Err(anyhow!(
                "Did not find adapter that can render to surface."
            ));
        };

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await?;

        let size = window.inner_size();
        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .ok_or_else(|| {
                anyhow!("Could not acquire default surface configuration.")
            })?;
        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
        })
    }

    fn render(&self) -> anyhow::Result<()> {
        let surface_texture = self.surface.get_current_texture()?;
        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}
