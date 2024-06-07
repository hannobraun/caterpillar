use capi_runtime::debugger::DebugEvent;

pub use capi_runtime::runner::EventsTx;

pub async fn send_event(event: DebugEvent, events: EventsTx) {
    if let Err(err) = events.send(event) {
        log::error!("Error sending event: {err}");
    }
}
