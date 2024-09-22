use capi_compiler::fragments::{Fragment, Hash};

#[derive(Clone)]
pub enum UserAction {
    BreakpointClear { fragment: Hash<Fragment> },
    BreakpointSet { fragment: Hash<Fragment> },
    Continue,
    Reset,
    StepIn,
    StepOut,
    StepOver,
    Stop,
}
