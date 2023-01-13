mod html;
mod window;

fn main() {
    let id = 1;

    let _ = window::Window::new(id);
    html::render();
}
