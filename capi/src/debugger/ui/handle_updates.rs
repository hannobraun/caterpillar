use leptos::{SignalSet, WriteSignal};

use crate::{debugger::model::Debugger, updates::UpdatesRx};

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
