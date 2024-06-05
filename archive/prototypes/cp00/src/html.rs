use sycamore::prelude::*;

use crate::language::Interpreter;

pub fn render(canvas_id: u32, interpreter: Interpreter) {
    sycamore::render(|cx| {
        let code = create_signal(cx, String::new());
        create_effect(cx, move || {
            interpreter.interpret(&code.get());
        });

        view! { cx,
            div(class="editor") {
                textarea(bind:value=code) {}
            }
            canvas(data-raw-handle=(canvas_id)) {}
        }
    });
}
