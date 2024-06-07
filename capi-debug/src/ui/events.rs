use capi_runtime::debugger::DebugEvent;
use futures::channel::mpsc;

pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;
pub type EventsRx = mpsc::UnboundedReceiver<DebugEvent>;
