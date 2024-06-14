use leptos::{create_signal, SignalSet, WriteSignal};

use crate::{
    debugger::{model::Debugger, ui::components::debugger::Debugger},
    updates::UpdatesRx,
};

use super::EventsTx;

pub fn start(updates_rx: UpdatesRx, events_tx: EventsTx) {
    let (debugger_read, debugger_write) = create_signal(None);

    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger debugger=debugger_read events=events_tx />
        }
    });

    leptos::spawn_local(handle_updates(updates_rx, debugger_write));
}

async fn handle_updates(
    mut updates: UpdatesRx,
    debugger_write: WriteSignal<Option<Debugger>>,
) {
    let mut debugger = Debugger { process: None };

    loop {
        let process = match updates.changed().await {
            Ok(()) => updates.borrow_and_update().clone(),
            Err(err) => panic!("{err}"),
        };

        debugger.process = Some(process);

        debugger_write.set(Some(debugger.clone()));
    }
}
