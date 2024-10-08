use capi_compiler::fragments::{FragmentId, FragmentLocation};

#[derive(Clone)]
pub enum UserAction {
    BreakpointClear {
        fragment: (FragmentId, FragmentLocation),
    },
    BreakpointSet {
        fragment: FragmentId,
    },
    Continue,
    Reset,
    StepIn,
    StepOut,
    StepOver,
    Stop,
}
