use sycamore::{reactive::RcSignal, view};

pub async fn render(output: RcSignal<String>) {
    sycamore::render(|cx| {
        view! { cx,
            ul {
                li { (output.get()) }
            }
        }
    });
}
