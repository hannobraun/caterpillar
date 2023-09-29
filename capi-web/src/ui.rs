use sycamore::{reactive::RcSignal, view};

pub async fn render(output: RcSignal<String>) -> anyhow::Result<()> {
    sycamore::render(|cx| {
        view! { cx,
            ul {
                li { (output.get()) }
            }
        }
    });

    Ok(())
}
