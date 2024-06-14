use leptos::{create_signal, SignalSet};

use crate::{
    debugger::{model::Debugger, ui::components::debugger::Debugger},
    updates::UpdatesRx,
};

use super::EventsTx;

pub fn start(mut updates_rx: UpdatesRx, events_tx: EventsTx) {
    let mut debugger = Debugger::new();
    let (debugger_read, debugger_write) = create_signal(debugger.clone());

    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger debugger=debugger_read events=events_tx />
        }
    });

    leptos::spawn_local(async move {
        loop {
            let Some(process) = updates_rx.recv().await else {
                // This means the other end has hung up. Nothing we can do,
                // except end this task too.
                break;
            };

            debugger.update_from_process(process);

            debugger_write.set(debugger.clone());
        }
    });
}
