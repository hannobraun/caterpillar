use sycamore::view;

pub fn render() {
    sycamore::render(|cx| {
        view! { cx,
            p() {
                "Hello, world!"
            }
        }
    });
}
