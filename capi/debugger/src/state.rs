use capi_process::Instructions;
use capi_protocol::{
    command::{CommandToRuntime, SerializedCommandToRuntime},
    updates::{Code, SerializedUpdate, UpdateFromRuntime},
    Versioned,
};
use gloo_net::http::{Request, Response};
use leptos::SignalSet;
use tokio::{
    select,
    sync::{
        mpsc::{self, UnboundedSender},
        watch,
    },
};

use crate::{
    model::{CodeRx, CodeTx, PersistentState},
    ui::{self, Action},
};

pub struct DebuggerState {
    pub code_rx: CodeRx,
    pub updates_from_runtime_tx: mpsc::UnboundedSender<SerializedUpdate>,
    pub commands_to_runtime_rx:
        mpsc::UnboundedReceiver<SerializedCommandToRuntime>,
}

impl DebuggerState {
    pub fn new() -> Self {
        let (code_tx, code_rx) = watch::channel(Instructions::default());
        let (updates_from_runtime_tx, mut updates_from_runtime_rx) =
            mpsc::unbounded_channel();
        let (commands_to_runtime_tx, commands_to_runtime_rx) =
            mpsc::unbounded_channel();
        let (actions_tx, mut actions_rx) = mpsc::unbounded_channel();

        let mut persistent = PersistentState::default();
        let transient = persistent.update();

        let (state_read, state_write) =
            leptos::create_signal((persistent.clone(), transient));

        leptos::spawn_local(async move {
            let code = Request::get("/code").send().await;
            let mut timestamp =
                on_new_code(code, &code_tx, &mut persistent).await;

            loop {
                let response =
                    Request::get(&format!("/code/{timestamp}")).send();

                select! {
                    code = response => {
                        timestamp =
                            on_new_code(
                                code,
                                &code_tx,
                                &mut persistent,
                            )
                            .await;
                    }
                    update = updates_from_runtime_rx.recv() => {
                        let Some(update) = update else {
                            // This means the other end has hung up. Nothing we
                            // can do, except end this task too.
                            break;
                        };

                        on_update_from_runtime(
                            update,
                            &mut persistent,
                        );
                    }
                    action = actions_rx.recv() => {
                        let Some(action) = action else {
                            // This means the other end has hung up. Nothing we
                            // can do, except end this task too.
                            break;
                        };

                        on_ui_action(
                            action,
                            &mut persistent,
                            &commands_to_runtime_tx,
                        );
                    }
                }

                let transient = persistent.update();
                state_write.set((persistent.clone(), transient));
            }
        });

        ui::init(state_read, actions_tx);

        Self {
            updates_from_runtime_tx,
            code_rx,
            commands_to_runtime_rx,
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
    code_tx: &CodeTx,
    state: &mut PersistentState,
) -> u64 {
    let code = code.unwrap().text().await.unwrap();
    let code: Versioned<Code> = ron::from_str(&code).unwrap();

    code_tx
        .send(code.inner.instructions.clone())
        .expect("Code receiver should never drop.");

    state.code = Some(code.inner.clone());

    code.timestamp
}

fn on_update_from_runtime(update: Vec<u8>, state: &mut PersistentState) {
    let update = UpdateFromRuntime::deserialize(update);
    state.on_update_from_runtime(update);
}

fn on_ui_action(
    action: Action,
    state: &mut PersistentState,
    commands_to_runtime_tx: &UnboundedSender<SerializedCommandToRuntime>,
) {
    let command = match action {
        Action::BreakpointClear { address } => {
            state.clear_durable_breakpoint(&address).expect(
                "Failed to clear durable breakpoint from the UI. This is a bug \
                in the Caterpillar debugger",
            );

            CommandToRuntime::BreakpointClear {
                instruction: address,
            }
        }
        Action::BreakpointSet { address } => {
            state.set_durable_breakpoint(address).expect(
                "Failed to set durable breakpoint from the UI. This is a bug \
                in the Caterpillar debugger",
            );

            CommandToRuntime::BreakpointSet {
                instruction: address,
            }
        }
        Action::Continue => CommandToRuntime::Continue,
        Action::Reset => CommandToRuntime::Reset,
        Action::Step => CommandToRuntime::Step,
        Action::Stop => CommandToRuntime::Stop,
    };

    commands_to_runtime_tx.send(command.serialize()).unwrap();
}
