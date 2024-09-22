use capi_compiler::fragments::{Fragment, FragmentId, Hash};

#[derive(Clone)]
pub enum UserAction {
    BreakpointClear { fragment: FragmentId },
    BreakpointSet { fragment: Hash<Fragment> },
    Continue,
    Reset,
    StepIn,
    StepOut,
    StepOver,
    Stop,
}
