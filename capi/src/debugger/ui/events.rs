use tokio::sync::mpsc;

use crate::debugger::model::DebugCommand;

pub type CommandsRx = mpsc::UnboundedReceiver<DebugCommand>;
pub type CommandsTx = mpsc::UnboundedSender<DebugCommand>;

pub async fn send_event(command: DebugCommand, events: CommandsTx) {
    if let Err(err) = events.send(command) {
        log::error!("Error sending event: {err}");
    }
}
