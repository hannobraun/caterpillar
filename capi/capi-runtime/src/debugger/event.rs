use crate::InstructionAddress;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugEvent {
    Continue,
    Reset,
    Step,
    ToggleBreakpoint { address: InstructionAddress },
}
