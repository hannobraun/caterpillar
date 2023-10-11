use crate::{
    runtime::{call_stack::CallStack, data_stack::DataStack},
    DataStackResult,
};

use super::UserDefined;

#[derive(Debug)]
pub enum NativeFunction<C> {
    Intrinsic(IntrinsicFunction),
    Platform(PlatformFunction<C>),
}

pub type IntrinsicFunction = fn(RuntimeContext) -> DataStackResult<()>;
pub type PlatformFunction<C> =
    fn(RuntimeContext, &mut C) -> DataStackResult<FunctionState>;

pub struct RuntimeContext<'r> {
    pub functions: UserDefined<'r>,
    pub call_stack: &'r mut CallStack,
    pub data_stack: &'r mut DataStack,
}

pub enum FunctionState {
    Done,
    Sleeping,
}
