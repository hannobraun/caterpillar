use crate::{
    builtins::types::{BuiltinContext, CoreBuiltinState, PlatformBuiltinState},
    runtime::data_stack::DataStackResult,
};

#[derive(Debug)]
pub enum NativeFunction<C> {
    Intrinsic(IntrinsicFunction),
    Platform(PlatformFunction<C>),
}

pub type IntrinsicFunction =
    fn(step: usize, BuiltinContext) -> DataStackResult<CoreBuiltinState>;
pub type PlatformFunction<C> =
    fn(BuiltinContext, &mut C) -> DataStackResult<PlatformBuiltinState>;
