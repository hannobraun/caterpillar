use capi_protocol::update::SerializedUpdate;
use tokio::sync::mpsc;

use crate::ui::CommandsRx;

pub struct DebuggerState {
    pub updates_tx: mpsc::UnboundedSender<SerializedUpdate>,
    pub commands_rx: CommandsRx,
}
