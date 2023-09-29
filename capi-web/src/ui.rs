use async_channel::Receiver;
use sycamore::{reactive::create_rc_signal, view};

pub async fn render(output_channel: Receiver<String>) -> anyhow::Result<()> {
    let output_signal = create_rc_signal(String::new());

    sycamore::render(|cx| {
        let output = output_signal.clone();

        view! { cx,
            ul {
                li { (output.get()) }
            }
        }
    });

    loop {
        let line = output_channel.recv().await?;
        output_signal.set(line);
    }
}
