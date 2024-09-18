use capi_compiler::fragments::FragmentId;

#[allow(dead_code)]
#[derive(Clone)]
pub enum UserAction {
    BreakpointClear { fragment: FragmentId },
    BreakpointSet { fragment: FragmentId },
    Continue,
    Reset,
    StepIn,
    StepOver,
    Stop,
}
