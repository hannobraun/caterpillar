use capi_compiler::fragments::Hash;

#[derive(Clone)]
pub enum UserAction {
    BreakpointClear { fragment: Hash },
    BreakpointSet { fragment: Hash },
    Continue,
    Reset,
    StepIn,
    StepOut,
    StepOver,
    Stop,
}
