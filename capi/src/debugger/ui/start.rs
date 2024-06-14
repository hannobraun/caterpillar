use leptos::{create_signal, SignalSet, WriteSignal};

use crate::{
    debugger::{model::Debugger, ui::components::debugger::Debugger},
    updates::UpdatesRx,
};

use super::EventsTx;

pub fn start(updates_rx: UpdatesRx, events_tx: EventsTx) {
    let (debugger, set_debugger) = create_signal(None);

    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger debugger=debugger events=events_tx />
        }
    });

    leptos::spawn_local(handle_updates(updates_rx, set_debugger));
}

pub async fn handle_updates(
    mut updates: UpdatesRx,
    set_debugger: WriteSignal<Option<Debugger>>,
) {
    loop {
        let process = match updates.changed().await {
            Ok(()) => updates.borrow_and_update().clone(),
            Err(err) => panic!("{err}"),
        };

        let debugger = Debugger { process };

        set_debugger.set(Some(debugger));
    }
}
