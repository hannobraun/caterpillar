use capi_compiler::fragments::FragmentId;

#[derive(Clone)]
pub enum UserAction {
    BreakpointClear { fragment: FragmentId },
    BreakpointSet { fragment: FragmentId },
    Continue,
    Reset,
    StepInto,
    Stop,
}
