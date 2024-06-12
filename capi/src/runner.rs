use tokio::sync::mpsc;

use crate::{debugger::DebugEvent, program::Program, updates::UpdatesTx};

pub fn runner(program: Program, updates: UpdatesTx) -> Runner {
    Runner { program, updates }
}

pub type EventsTx = mpsc::UnboundedSender<DebugEvent>;

pub struct Runner {
    pub program: Program,
    pub updates: UpdatesTx,
}
