use crate::InstructionAddress;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugEvent {
    Continue {
        and_stop_at: Option<InstructionAddress>,
    },
    Reset,
    Step,
    Stop,
    ToggleBreakpoint {
        address: InstructionAddress,
    },
}
