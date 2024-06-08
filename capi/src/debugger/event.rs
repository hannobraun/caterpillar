use crate::runtime;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugEvent {
    Continue {
        and_stop_at: Option<runtime::Location>,
    },
    Reset,
    Step,
    Stop,
    ToggleBreakpoint {
        location: runtime::Location,
    },
}
