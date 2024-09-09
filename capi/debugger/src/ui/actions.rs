use capi_process::InstructionAddress;
use capi_protocol::command::CommandToRuntime;
use tokio::sync::mpsc;

pub type ActionsTx = mpsc::UnboundedSender<CommandToRuntime>;

#[derive(Clone)]
pub enum Action {
    BreakpointClear { instruction: InstructionAddress },
    BreakpointSet { instruction: InstructionAddress },
    Continue,
    Reset,
    Step,
    Stop,
}
