use sycamore::prelude::*;

pub fn render(canvas_id: u32) {
    sycamore::render(|cx| {
        let signal = create_signal(cx, String::new());
        create_effect(cx, || log::warn!("{}", signal.get()));

        view! { cx,
            textarea(bind:value=signal) {}
            canvas(data-raw-handle=(canvas_id)) {}
        }
    });
}
