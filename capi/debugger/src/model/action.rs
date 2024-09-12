use capi_compiler::fragments::FragmentId;

#[derive(Clone)]
pub enum Action {
    BreakpointClear { fragment: FragmentId },
    BreakpointSet { fragment: FragmentId },
    Continue,
    Reset,
    Step,
    Stop,
}
