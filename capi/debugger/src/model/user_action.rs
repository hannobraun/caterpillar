use capi_compiler::fragments::FragmentLocation;

#[derive(Clone)]
pub enum UserAction {
    BreakpointClear { fragment: FragmentLocation },
    BreakpointSet { fragment: FragmentLocation },
    Continue,
    Reset,
    StepIn,
    StepOut,
    StepOver,
    Stop,
}
