use crate::{
    repr::eval::fragments::{FragmentId, Fragments},
    runtime::{call_stack::CallStack, data_stack::DataStack},
    DataStackResult,
};

use super::UserDefinedItems;

#[derive(Debug)]
pub enum NativeFunction<C> {
    Intrinsic(IntrinsicFunction),
    Platform(PlatformFunction<C>),
}

pub type IntrinsicFunction =
    fn(step: usize, RuntimeContext) -> DataStackResult<IntrinsicFunctionState>;
pub type PlatformFunction<C> =
    fn(RuntimeContext, &mut C) -> DataStackResult<PlatformFunctionState>;

pub struct RuntimeContext<'r> {
    pub this: FragmentId,
    pub fragments: &'r mut Fragments,
    pub namespace: UserDefinedItems<'r>,
    pub call_stack: &'r mut CallStack,
    pub data_stack: &'r mut DataStack,
}

pub enum IntrinsicFunctionState {
    FullyCompleted,
}

pub enum PlatformFunctionState {
    Done,
    Sleeping,
}
