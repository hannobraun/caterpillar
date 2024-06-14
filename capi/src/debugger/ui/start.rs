use leptos::{create_signal, SignalSet};

use crate::{
    debugger::{model::Debugger, ui::components::debugger::Debugger},
    updates::UpdatesRx,
};

use super::EventsTx;

pub fn start(mut updates_rx: UpdatesRx, events_tx: EventsTx) {
    let mut debugger = Debugger { process: None };
    let (debugger_read, debugger_write) = create_signal(debugger.clone());

    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger debugger=debugger_read events=events_tx />
        }
    });

    leptos::spawn_local(async move {
        loop {
            let process = match updates_rx.changed().await {
                Ok(()) => updates_rx.borrow_and_update().clone(),
                Err(err) => panic!("{err}"),
            };

            debugger.process = Some(process);

            debugger_write.set(debugger.clone());
        }
    });
}
