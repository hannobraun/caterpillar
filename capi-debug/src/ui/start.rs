use capi_runtime::Program;
use leptos::{create_signal, WriteSignal};

use crate::ui::components::debugger::Debugger;

use super::EventsTx;

pub fn start(events_tx: EventsTx) -> WriteSignal<Option<Program>> {
    let (program, set_program) = create_signal(None);

    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger program=program events=events_tx />
        }
    });

    set_program
}
