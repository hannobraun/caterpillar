use capi_process::InstructionAddress;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Command {
    BreakpointClear { instruction: InstructionAddress },
    BreakpointSet { instruction: InstructionAddress },
    Continue,
    Reset,
    Step,
    Stop,
}
