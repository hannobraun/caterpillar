use capi_runtime::{debugger::DebugEvent, Program};
use futures::channel::mpsc::{self, UnboundedReceiver};
use leptos::{create_signal, WriteSignal};

use crate::ui::components::debugger::Debugger;

pub fn start() -> (WriteSignal<Option<Program>>, UnboundedReceiver<DebugEvent>)
{
    let (program, set_program) = create_signal(None);
    let (events_tx, events_rx) = mpsc::unbounded();

    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger program=program events=events_tx />
        }
    });

    (set_program, events_rx)
}
