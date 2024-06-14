use leptos::{SignalSet, WriteSignal};

use crate::{process::Process, updates::UpdatesRx};

pub async fn handle_updates(
    mut updates: UpdatesRx,
    set_process: WriteSignal<Option<Process>>,
) {
    loop {
        let process = match updates.changed().await {
            Ok(()) => updates.borrow_and_update().clone(),
            Err(err) => panic!("{err}"),
        };

        set_process.set(Some(process));
    }
}
