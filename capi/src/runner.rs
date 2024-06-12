use tokio::sync::mpsc;

use crate::{debugger::DebugEvent, program::Program};

pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;

pub struct Runner {
    pub program: Program,
}
