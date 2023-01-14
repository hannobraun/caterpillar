mod html;
mod window;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init().unwrap();

    let id = 1;

    let _ = window::Window::new(id);
    html::render(id);
}
