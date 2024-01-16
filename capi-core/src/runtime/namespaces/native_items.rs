use crate::{
    module::Module,
    repr::eval::fragments::FragmentId,
    runtime::{call_stack::CallStack, data_stack::DataStack},
    DataStackResult,
};

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
    /// The fragment ID of the word that refers to this intrinsic or platform fn
    pub word: FragmentId,

    pub namespace: &'r mut Module,
    pub call_stack: &'r mut CallStack,
    pub data_stack: &'r mut DataStack,
    pub side_stack: &'r mut DataStack,
}

pub enum IntrinsicFunctionState {
    StepDone,
    FullyCompleted,
}

pub enum PlatformFunctionState {
    Done,
    Sleeping,
}
