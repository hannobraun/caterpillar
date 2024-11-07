use capi_compiler::code::ExpressionLocation;

#[derive(Clone)]
pub enum UserAction {
    BreakpointClear { expression: ExpressionLocation },
    BreakpointSet { expression: ExpressionLocation },
    Continue,
    Reset,
    StepIn,
    StepOut,
    StepOver,
    Stop,
}
