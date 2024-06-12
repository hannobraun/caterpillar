use tokio::sync::mpsc;

use crate::{debugger::DebugEvent, program::Program, updates::UpdatesTx};

pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;

pub struct Runner {
    pub program: Program,
    pub updates_tx: UpdatesTx,
}
