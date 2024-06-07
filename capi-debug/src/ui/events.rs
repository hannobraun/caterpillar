use capi_runtime::debugger::DebugEvent;
use futures::{channel::mpsc, SinkExt};

pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;
pub type EventsRx = mpsc::UnboundedReceiver<DebugEvent>;

pub async fn send_event(event: DebugEvent, mut events: EventsTx) {
    if let Err(err) = events.send(event).await {
        log::error!("Error sending event: {err}");
    }
}
