use capi_runtime::{debugger::DebugEvent, Program};
use futures::channel::mpsc::UnboundedSender;
use leptos::{create_signal, WriteSignal};

use crate::ui::components::debugger::Debugger;

pub fn start(
    events_tx: UnboundedSender<DebugEvent>,
) -> WriteSignal<Option<Program>> {
    let (program, set_program) = create_signal(None);

    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger program=program events=events_tx />
        }
    });

    set_program
}
