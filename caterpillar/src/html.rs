use sycamore::prelude::*;

pub fn render(canvas_id: u32) {
    sycamore::render(|cx| {
        view! { cx,
            textarea {}
            canvas(data-raw-handle=(canvas_id)) {}
        }
    });
}
