use capi_protocol::{
    updates::{Code, SerializedUpdate, Update},
    Versioned,
};
use gloo_net::http::Request;
use leptos::{create_signal, SignalSet, WriteSignal};
use tokio::sync::mpsc;

use crate::{
    debugger::{Debugger, RemoteProcess},
    ui::components::debugger::Debugger,
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
        let code = Request::get("/code").send().await.unwrap();
        let code = code.text().await.unwrap();
        let code: Versioned<Code> = ron::from_str(&code).unwrap();

        remote_process.on_code_update(code.inner);

        loop {
            let Some(update) = updates_rx.recv().await else {
                // This means the other end has hung up. Nothing we can do,
                // except end this task too.
                break;
            };

            on_update(update, &mut remote_process, &debugger_write)
        }
    });
}

fn on_update(
    update: Vec<u8>,
    remote_process: &mut RemoteProcess,
    debugger_write: &WriteSignal<Debugger>,
) {
    let update = Update::deserialize(update);
    remote_process.on_runtime_update(update);

    debugger_write.set(remote_process.to_debugger());
}
