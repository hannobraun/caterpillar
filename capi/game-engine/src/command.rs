use capi_process::Instructions;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum Command {
    UpdateCode { instructions: Instructions },
    Continue,
    IgnoreNextInstruction,
    Reset,
    Step,
    Stop,
}
