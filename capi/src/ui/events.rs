use tokio::sync::mpsc;

use crate::debugger::DebugEvent;

pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;

pub async fn send_event(event: DebugEvent, events: EventsTx) {
    if let Err(err) = events.send(event) {
        log::error!("Error sending event: {err}");
    }
}
