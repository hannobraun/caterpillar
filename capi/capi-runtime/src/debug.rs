use crate::InstructionAddress;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugEvent {
    Reset,
    ToggleBreakpoint { address: InstructionAddress },
}
