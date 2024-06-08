use crate::debugger::DebugEvent;
pub use crate::runner::EventsTx;

pub async fn send_event(event: DebugEvent, events: EventsTx) {
    if let Err(err) = events.send(event) {
        log::error!("Error sending event: {err}");
    }
}
