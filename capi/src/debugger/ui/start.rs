use leptos::{create_signal, SignalSet};

use crate::{
    debugger::{
        remote_process::RemoteProcess, ui::components::debugger::Debugger,
    },
    updates::{Update, UpdatesRx},
};

use super::EventsTx;

pub fn start(mut updates_rx: UpdatesRx, events_tx: EventsTx) {
    let mut remote_process = RemoteProcess::new();
    let (debugger_read, debugger_write) =
        create_signal(remote_process.to_debugger());

    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger debugger=debugger_read events=events_tx />
        }
    });

    leptos::spawn_local(async move {
        loop {
            let Some(update) = updates_rx.recv().await else {
                // This means the other end has hung up. Nothing we can do,
                // except end this task too.
                break;
            };

            let update = Update::deserialize(update);
            remote_process.on_update(update);

            debugger_write.set(remote_process.to_debugger());
        }
    });
}
