use async_channel::Receiver;
use sycamore::{reactive::RcSignal, view};

pub async fn render(
    output: RcSignal<String>,
    output2: Receiver<String>,
) -> anyhow::Result<()> {
    sycamore::render(|cx| {
        view! { cx,
            ul {
                li { (output.get()) }
            }
        }
    });

    loop {
        let line = output2.recv().await?;
        tracing::debug!("Output: {line}");
    }
}
