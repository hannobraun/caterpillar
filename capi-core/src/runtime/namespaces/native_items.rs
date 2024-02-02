use crate::{
    builtins::types::{BuiltinContext, CoreBuiltinState, PlatformBuiltin},
    runtime::data_stack::DataStackResult,
};

#[derive(Debug)]
pub enum NativeFunction<C> {
    Intrinsic(IntrinsicFunction),
    Platform(PlatformBuiltin<C>),
}

pub type IntrinsicFunction =
    fn(step: usize, BuiltinContext) -> DataStackResult<CoreBuiltinState>;
