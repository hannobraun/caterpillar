use async_channel::Receiver;
use sycamore::{
    reactive::{create_effect, create_rc_signal},
    view,
};

pub async fn render(
    script: &str,
    output_channel: Receiver<String>,
) -> anyhow::Result<()> {
    let script = script.to_string();
    let output_signal = create_rc_signal(String::new());

    sycamore::render(|cx| {
        let output = output_signal.clone();

        let output2 = output.clone();
        create_effect(cx, move || {
            output2.track();

            let document = web_sys::window().unwrap().document().unwrap();
            let outputs = document.get_elements_by_class_name("output");

            let mut i = 0;
            while let Some(output) = outputs.item(i) {
                output.set_scroll_top(output.scroll_height());
                i += 1;
            }
        });

        view! { cx,
            textarea(readonly=true) {
                (script)
            }
            textarea(class="output", readonly=true) {
                (output.get())
            }
        }
    });

    loop {
        let line = output_channel.recv().await?;
        output_signal.modify().push_str(&line);
    }
}
