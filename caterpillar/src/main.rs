use std::{cell::RefCell, rc::Rc};

use futures::executor::block_on;
use wasm_bindgen::{prelude::Closure, JsCast};

mod html;
mod renderer;
mod window;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init().unwrap();

    let id = 1;

    let window = window::Window::new(id);
    html::render(id);

    block_on(render(window));

    let main_loop: Rc<RefCell<Option<Closure<dyn FnMut()>>>> =
        Rc::new(RefCell::new(None));
    let main_loop_2 = main_loop.clone();

    *main_loop_2.borrow_mut() = Some(Closure::new(move || {
        log::info!("requestAnimationFrame fired");
        request_animation_frame(&main_loop);
    }));

    request_animation_frame(&main_loop_2)
}

fn request_animation_frame(f: &Rc<RefCell<Option<Closure<dyn FnMut()>>>>) {
    if let Some(window) = web_sys::window() {
        window
            .request_animation_frame(
                f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            )
            .unwrap();
    }
}

async fn render(window: window::Window) {
    let renderer = renderer::Renderer::new(&window).await.unwrap();
    renderer.draw([0., 0., 0., 1.]).unwrap();
}
