use capi_protocol::{
    updates::{Code, SerializedUpdate, Update},
    Versioned,
};
use gloo_net::http::{Request, Response};
use leptos::SignalSet;
use tokio::{select, sync::mpsc};

use crate::{
    debugger::RemoteProcess,
    ui::{self, CommandsRx},
};

pub struct DebuggerState {
    pub updates_tx: mpsc::UnboundedSender<SerializedUpdate>,
    pub commands_rx: CommandsRx,
}

impl DebuggerState {
    pub fn new() -> Self {
        let (commands_tx, commands_rx) = mpsc::unbounded_channel();
        let (updates_tx, mut updates_rx) = mpsc::unbounded_channel();

        let mut remote_process = RemoteProcess::default();
        let (debugger_read, debugger_write) =
            leptos::create_signal(remote_process.to_debugger());

        leptos::spawn_local(async move {
            let code = Request::get("/code").send().await;
            let mut timestamp = on_new_code(code, &mut remote_process).await;

            loop {
                let response =
                    Request::get(&format!("/code/{timestamp}")).send();

                select! {
                    code = response => {
                        timestamp =
                            on_new_code(code, &mut remote_process).await;
                    }
                    update = updates_rx.recv() => {
                        let Some(update) = update else {
                            // This means the other end has hung up. Nothing we
                            // can do, except end this task too.
                            break;
                        };

                        on_process_update(
                            update,
                            &mut remote_process,
                        );
                    }
                }

                debugger_write.set(remote_process.to_debugger());
            }
        });

        ui::start(debugger_read, commands_tx);

        Self {
            updates_tx,
            commands_rx,
        }
    }
}

impl Default for DebuggerState {
    fn default() -> Self {
        Self::new()
    }
}

async fn on_new_code(
    code: Result<Response, gloo_net::Error>,
    remote_process: &mut RemoteProcess,
) -> u64 {
    let code = code.unwrap().text().await.unwrap();
    let code: Versioned<Code> = ron::from_str(&code).unwrap();

    remote_process.on_code_update(code.inner);

    code.timestamp
}

fn on_process_update(update: Vec<u8>, remote_process: &mut RemoteProcess) {
    let update = Update::deserialize(update);
    remote_process.on_runtime_update(update);
}
