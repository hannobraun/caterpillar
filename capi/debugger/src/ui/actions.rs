use capi_process::InstructionAddress;
use tokio::sync::mpsc;

pub type ActionsTx = mpsc::UnboundedSender<Action>;

#[derive(Clone)]
pub enum Action {
    BreakpointClear { instruction: InstructionAddress },
    BreakpointSet { instruction: InstructionAddress },
    Continue,
    Reset,
    Step,
    Stop,
}
