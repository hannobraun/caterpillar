use tokio::sync::mpsc;

use crate::debugger::model::DebugCommand;

pub type CommandsRx = mpsc::UnboundedReceiver<DebugCommand>;
pub type CommandsTx = mpsc::UnboundedSender<DebugCommand>;

pub async fn send_event(event: DebugCommand, events: CommandsTx) {
    if let Err(err) = events.send(event) {
        log::error!("Error sending event: {err}");
    }
}
