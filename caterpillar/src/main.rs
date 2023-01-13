mod html;
mod window;

fn main() {
    console_error_panic_hook::set_once();

    let id = 1;

    let _ = window::Window::new(id);
    html::render(id);
}
