use sycamore::prelude::*;

pub fn render() {
    sycamore::render(|cx| {
        view! { cx,
            textarea {}
            canvas {}
        }
    });
}
