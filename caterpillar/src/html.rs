use sycamore::prelude::*;

use crate::language::Language;

pub fn render(canvas_id: u32, language: Language) {
    sycamore::render(|cx| {
        let signal = create_signal(cx, String::new());
        create_effect(cx, move || {
            let Ok(value) = signal.get().parse::<u8>() else {
                return
            };
            let value = value as f64 / u8::MAX as f64;

            *language.background_color.borrow_mut() = [value, value, value, 1.];
        });

        view! { cx,
            textarea(bind:value=signal) {}
            canvas(data-raw-handle=(canvas_id)) {}
        }
    });
}
