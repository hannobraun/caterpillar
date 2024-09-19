use capi_process::Instructions;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Command {
    UpdateCode { instructions: Instructions },
    ClearBreakpointAndEvaluateNextInstruction,
    ClearBreakpointAndContinue,
    Reset,
    Stop,
}
