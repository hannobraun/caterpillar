use capi_compiler::fragments::{FragmentId, FragmentLocation};

#[derive(Clone)]
pub enum UserAction {
    BreakpointClear {
        fragment: FragmentLocation,
    },
    BreakpointSet {
        fragment: (FragmentId, FragmentLocation),
    },
    Continue,
    Reset,
    StepIn,
    StepOut,
    StepOver,
    Stop,
}
