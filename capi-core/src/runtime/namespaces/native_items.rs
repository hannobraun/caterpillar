use crate::{
    builtins::types::BuiltinContext, runtime::data_stack::DataStackResult,
};

#[derive(Debug)]
pub enum NativeFunction<C> {
    Intrinsic(IntrinsicFunction),
    Platform(PlatformFunction<C>),
}

pub type IntrinsicFunction =
    fn(step: usize, BuiltinContext) -> DataStackResult<IntrinsicFunctionState>;
pub type PlatformFunction<C> =
    fn(BuiltinContext, &mut C) -> DataStackResult<PlatformFunctionState>;

pub enum IntrinsicFunctionState {
    StepDone,
    FullyCompleted,
}

pub enum PlatformFunctionState {
    Done,
    Sleeping,
}
