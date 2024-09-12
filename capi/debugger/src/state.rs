use capi_process::Instructions;
use capi_protocol::{
    command::SerializedCommandToRuntime,
    updates::{SerializedUpdate, UpdateFromRuntime},
};
use leptos::SignalSet;
use tokio::{
    select,
    sync::{
        mpsc::{self, UnboundedSender},
        watch,
    },
};

use crate::{
    code::{CodeManager, CodeRx},
    model::{UserAction, PersistentState},
    ui,
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
        let transient = persistent.generate_transient_state();

        let (state_read, state_write) =
            leptos::create_signal((persistent.clone(), transient));

        leptos::spawn_local(async move {
            let mut code =
                CodeManager::new(&code_tx, &mut persistent).await.unwrap();

            loop {
                select! {
                    result =
                        code.wait_for_new_code(&code_tx, &mut persistent)
                    => {
                        result.unwrap();

                        // Nothing else to do, except do the update that happens
                        // below this `select!`.
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

                let transient = persistent.generate_transient_state();
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

fn on_update_from_runtime(update: Vec<u8>, state: &mut PersistentState) {
    let update = UpdateFromRuntime::deserialize(update);
    state.on_update_from_runtime(update);
}

fn on_ui_action(
    action: UserAction,
    state: &mut PersistentState,
    commands_to_runtime_tx: &UnboundedSender<SerializedCommandToRuntime>,
) {
    let command = state.on_ui_action(action).expect(
        "Failed to handle UI action. This is most likely a bug in the \
        Caterpillar debugger:",
    );

    if let Some(command) = command {
        commands_to_runtime_tx.send(command.serialize()).unwrap();
    }
}
