use tokio::sync::mpsc;

use crate::debugger::DebugEvent;

pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;
