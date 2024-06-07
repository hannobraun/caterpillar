use capi_runtime::{debugger::DebugEvent, Program};
use futures::channel::mpsc::UnboundedSender;
use leptos::ReadSignal;

use crate::ui::components::debugger::Debugger;

pub fn start(
    program: ReadSignal<Option<Program>>,
    events_tx: UnboundedSender<DebugEvent>,
) {
    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger program=program events=events_tx />
        }
    });
}
