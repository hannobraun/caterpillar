use capi_process::Instructions;
use capi_protocol::{
    updates::{Code, SerializedUpdate, Update},
    Versioned,
};
use gloo_net::http::{Request, Response};
use leptos::SignalSet;
use tokio::{
    select,
    sync::{mpsc, watch},
};

use crate::{
    debugger::{Debugger, RemoteProcess},
    ui::{self, CommandsRx},
};

pub struct DebuggerState {
    pub code_rx: watch::Receiver<Instructions>,
    pub from_process_tx: mpsc::UnboundedSender<SerializedUpdate>,
    pub to_process_rx: CommandsRx,
}

impl DebuggerState {
    pub fn new() -> Self {
        let (code_tx, code_rx) = watch::channel(Instructions::default());
        let (to_process_tx, to_process_rx) = mpsc::unbounded_channel();
        let (from_process_tx, mut from_process_rx) = mpsc::unbounded_channel();

        let mut debugger = Debugger::default();
        let mut remote_process = RemoteProcess::default();

        let (debugger_read, debugger_write) =
            leptos::create_signal(debugger.clone());

        leptos::spawn_local(async move {
            let code = Request::get("/code").send().await;
            let mut timestamp =
                on_new_code(code, &code_tx, &mut remote_process).await;

            loop {
                let response =
                    Request::get(&format!("/code/{timestamp}")).send();

                select! {
                    code = response => {
                        timestamp =
                            on_new_code(
                                code,
                                &code_tx,
                                &mut remote_process,
                            )
                            .await;
                    }
                    update = from_process_rx.recv() => {
                        let Some(update) = update else {
                            // This means the other end has hung up. Nothing we
                            // can do, except end this task too.
                            break;
                        };

                        on_update_from_process(
                            update,
                            &mut remote_process,
                        );
                    }
                }

                debugger.update(
                    remote_process.code.clone(),
                    remote_process.memory.clone(),
                    remote_process.process.as_ref(),
                );
                debugger_write.set(debugger.clone());
            }
        });

        ui::start(debugger_read, to_process_tx);

        Self {
            from_process_tx,
            code_rx,
            to_process_rx,
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
    code_tx: &watch::Sender<Instructions>,
    remote_process: &mut RemoteProcess,
) -> u64 {
    let code = code.unwrap().text().await.unwrap();
    let code: Versioned<Code> = ron::from_str(&code).unwrap();

    code_tx
        .send(code.inner.instructions.clone())
        .expect("Code receiver should never drop.");

    remote_process.on_code_update(code.inner);

    code.timestamp
}

fn on_update_from_process(update: Vec<u8>, remote_process: &mut RemoteProcess) {
    let update = Update::deserialize(update);
    remote_process.on_runtime_update(update);
}
