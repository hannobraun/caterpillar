use wasm_bindgen_futures::spawn_local;

mod html;
mod renderer;
mod window;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init().unwrap();

    let id = 1;

    let window = window::Window::new(id);
    html::render(id);

    spawn_local(render(window));
}

async fn render(window: window::Window) {
    let renderer = renderer::Renderer::new(&window).await.unwrap();
    renderer.draw([0., 0., 0., 1.]).unwrap();
}
