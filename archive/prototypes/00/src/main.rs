use std::{cell::RefCell, rc::Rc};

use futures::executor::block_on;
use gloo::utils::window;
use wasm_bindgen::{prelude::Closure, JsCast};

mod html;
mod language;
mod renderer;
mod window;

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Warn).unwrap();

    let id = 1;

    let (language, background_color) = language::init();

    html::render(id, language);

    let window = window::Window::new(id);
    let renderer = block_on(renderer::Renderer::new(&window)).unwrap();

    let state = State {
        background_color,
        window,
        renderer,
    };

    main_loop(move || {
        let background_color = *state.background_color.borrow();

        state
            .renderer
            .draw(&state.window, background_color)
            .unwrap();
    });
}

pub struct State {
    background_color: language::Output,
    window: window::Window,
    renderer: renderer::Renderer,
}

type MainLoop = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

fn main_loop(mut f: impl FnMut() + 'static) {
    let main_loop: MainLoop = Rc::new(RefCell::new(None));
    let main_loop_2 = main_loop.clone();

    *main_loop_2.borrow_mut() = Some(Closure::new(move || {
        f();
        request_animation_frame(&main_loop);
    }));

    request_animation_frame(&main_loop_2)
}

fn request_animation_frame(main_loop: &MainLoop) {
    window()
        .request_animation_frame(
            main_loop
                .borrow()
                .as_ref()
                .unwrap()
                .as_ref()
                .unchecked_ref(),
        )
        .unwrap();
}
