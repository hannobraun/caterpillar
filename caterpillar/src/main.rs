use std::{cell::RefCell, rc::Rc};

use futures::executor::block_on;
use gloo::utils::window;
use wasm_bindgen::{prelude::Closure, JsCast};

mod html;
mod renderer;
mod window;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Warn).unwrap();

    let id = 1;

    let background_color = [0., 0., 0., 1.];

    html::render(id);

    let window = window::Window::new(id);
    let renderer = block_on(renderer::Renderer::new(&window)).unwrap();

    let state = State {
        background_color,
        window,
        renderer,
    };

    main_loop(move || {
        state
            .renderer
            .draw(&state.window, state.background_color)
            .unwrap();
    });
}

pub struct State {
    background_color: [f64; 4],
    window: window::Window,
    renderer: renderer::Renderer,
}

fn main_loop(mut f: impl FnMut() + 'static) {
    let main_loop: Rc<RefCell<Option<Closure<dyn FnMut()>>>> =
        Rc::new(RefCell::new(None));
    let main_loop_2 = main_loop.clone();

    *main_loop_2.borrow_mut() = Some(Closure::new(move || {
        f();
        request_animation_frame(&main_loop);
    }));

    request_animation_frame(&main_loop_2)
}

fn request_animation_frame(f: &Rc<RefCell<Option<Closure<dyn FnMut()>>>>) {
    window()
        .request_animation_frame(
            f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
        )
        .unwrap();
}
