use capi_protocol::update::{SerializedUpdate, Update};
use leptos::{create_signal, SignalSet};
use tokio::sync::mpsc;

use crate::debugger::{
    remote_process::RemoteProcess, ui::components::debugger::Debugger,
};

use super::CommandsTx;

pub fn start(
    mut updates_rx: mpsc::UnboundedReceiver<SerializedUpdate>,
    commands_tx: CommandsTx,
) {
    let mut remote_process = RemoteProcess::default();
    let (debugger_read, debugger_write) =
        create_signal(remote_process.to_debugger());

    leptos::mount_to_body(move || {
        leptos::view! {
            <Debugger debugger=debugger_read commands=commands_tx />
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
