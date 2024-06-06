use crate::runtime;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugEvent {
    Continue {
        and_stop_at: Option<runtime::InstructionAddress>,
    },
    Reset,
    Step,
    Stop,
    ToggleBreakpoint {
        address: runtime::InstructionAddress,
    },
}
