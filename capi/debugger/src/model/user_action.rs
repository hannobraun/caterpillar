use capi_compiler::code::ExpressionLocation;

#[derive(Clone)]
pub enum UserAction {
    BreakpointClear { fragment: ExpressionLocation },
    BreakpointSet { fragment: ExpressionLocation },
    Continue,
    Reset,
    StepIn,
    StepOut,
    StepOver,
    Stop,
}
