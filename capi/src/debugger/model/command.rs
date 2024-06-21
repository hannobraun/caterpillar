use crate::runtime;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum DebugCommand {
    BreakpointClear {
        location: runtime::Location,
    },
    BreakpointSet {
        location: runtime::Location,
    },
    Continue {
        and_stop_at: Option<runtime::Location>,
    },
    Reset,
    Step,
    Stop,
}
