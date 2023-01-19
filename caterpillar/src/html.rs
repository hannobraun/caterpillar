use std::{cell::RefCell, rc::Rc};

use sycamore::prelude::*;

pub fn render(canvas_id: u32, background_color: Rc<RefCell<[f64; 4]>>) {
    sycamore::render(|cx| {
        let signal = create_signal(cx, String::new());
        create_effect(cx, move || {
            let Ok(value) = signal.get().parse::<u8>() else {
                return
            };
            let value = value as f64 / u8::MAX as f64;

            *background_color.borrow_mut() = [value, value, value, 1.];
        });

        view! { cx,
            textarea(bind:value=signal) {}
            canvas(data-raw-handle=(canvas_id)) {}
        }
    });
}
