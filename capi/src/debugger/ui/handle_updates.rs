use leptos::{SignalSet, WriteSignal};

use crate::{debugger::model::Debugger, updates::UpdatesRx};

pub async fn handle_updates(
    mut updates: UpdatesRx,
    set_process: WriteSignal<Option<Debugger>>,
) {
    loop {
        let process = match updates.changed().await {
            Ok(()) => updates.borrow_and_update().clone(),
            Err(err) => panic!("{err}"),
        };

        let debugger = Debugger { process };

        set_process.set(Some(debugger));
    }
}
