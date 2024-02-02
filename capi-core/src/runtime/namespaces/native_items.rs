use crate::{
    builtins::types::{BuiltinContext, CoreBuiltinState, PlatformFunction},
    runtime::data_stack::DataStackResult,
};

#[derive(Debug)]
pub enum NativeFunction<C> {
    Intrinsic(IntrinsicFunction),
    Platform(PlatformFunction<C>),
}

pub type IntrinsicFunction =
    fn(step: usize, BuiltinContext) -> DataStackResult<CoreBuiltinState>;
