use capi_protocol::{
    update::{SerializedUpdate, SourceCode, Update},
    Versioned,
};
use gloo_net::http::Request;
use tokio::sync::mpsc;

use crate::ui::{self, CommandsRx};

pub struct DebuggerState {
    pub updates_tx: mpsc::UnboundedSender<SerializedUpdate>,
    pub commands_rx: CommandsRx,
}

impl DebuggerState {
    pub fn new() -> Self {
        let (commands_tx, commands_rx) = mpsc::unbounded_channel();
        let (updates_tx, updates_rx) = mpsc::unbounded_channel();

        let source_code_tx = updates_tx.clone();
        leptos::spawn_local(async move {
            let source_code =
                Request::get("/source-code/0").send().await.unwrap();
            let source_code = source_code.text().await.unwrap();
            let source_code: Versioned<SourceCode> =
                ron::from_str(&source_code).unwrap();

            let update = Update::SourceCode(source_code.inner);
            let update = ron::to_string(&update).unwrap().as_bytes().to_vec();

            source_code_tx.send(update).unwrap();
        });

        ui::start(updates_rx, commands_tx);

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
