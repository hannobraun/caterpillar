use capi_protocol::update::SerializedUpdate;
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
