use tokio::sync::mpsc;

use crate::debugger::model::DebugCommand;

pub type CommandsRx = mpsc::UnboundedReceiver<DebugCommand>;
pub type EventsTx = mpsc::UnboundedSender<DebugCommand>;

pub async fn send_event(event: DebugCommand, events: EventsTx) {
    if let Err(err) = events.send(event) {
        log::error!("Error sending event: {err}");
    }
}
