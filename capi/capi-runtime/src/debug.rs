use crate::{InstructionAddress, SourceLocation};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugEvent {
    ToggleBreakpoint {
        address: InstructionAddress,
        location: SourceLocation,
    },
}
