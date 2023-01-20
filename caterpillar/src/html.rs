use sycamore::prelude::*;

use crate::language::Interpreter;

pub fn render(canvas_id: u32, interpreter: Interpreter) {
    sycamore::render(|cx| {
        let value = create_signal(cx, String::new());
        create_effect(cx, move || {
            interpreter.interpret(&value.get());
        });

        view! { cx,
            textarea(bind:value=value) {}
            canvas(data-raw-handle=(canvas_id)) {}
        }
    });
}
