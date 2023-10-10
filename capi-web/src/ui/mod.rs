use async_channel::{Receiver, Sender};
use sycamore::{
    reactive::{create_effect, create_rc_signal, create_signal},
    view,
};

use crate::platform::Event;

pub async fn render(
    script: &str,
    code_channel: Sender<String>,
    output_channel: Receiver<Event>,
) -> anyhow::Result<()> {
    let script = script.to_string();
    let output_signal = create_rc_signal(String::new());

    sycamore::render(|cx| {
        let output = output_signal.clone();

        let code_signal = create_signal(cx, script.clone());
        create_effect(cx, move || {
            code_channel
                .send_blocking(code_signal.get().as_ref().clone())
                .unwrap();
        });

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
            div(class="h-screen flex flex-col") {
                div(class="basis-4/5") {
                    textarea(
                        class="h-full w-1/2 resize-none",
                        bind:value=code_signal,
                    ) {
                        (script)
                    }
                    textarea(
                        class="output h-full w-1/2 resize-none",
                        readonly=true,
                    ) {
                        (output.get())
                    }
                }
                textarea(class="basis-1/5 resize-none", readonly=true) {
                    "This is a placeholder. "
                    "Eventually, errors and status messages can be shown here."
                }
            }
        }
    });

    loop {
        let Event::Output(line) = output_channel.recv().await?;
        output_signal.modify().push_str(&line);
    }
}
