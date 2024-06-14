use crate::runtime;

#[derive(Clone, Debug)]
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
