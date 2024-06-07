use capi_runtime::debugger::DebugEvent;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};

pub type EventsTx = UnboundedSender<DebugEvent>;
pub type EventsRx = UnboundedReceiver<DebugEvent>;
