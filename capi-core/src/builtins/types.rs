use crate::{
    pipeline::Module,
    repr::eval::fragments::{FragmentId, Fragments},
    runtime::{call_stack::CallStack, data_stack::DataStack},
};

pub struct BuiltinContext<'r> {
    /// The fragment ID of the word that refers to this intrinsic or platform fn
    pub word: FragmentId,

    pub fragments: &'r mut Fragments,
    pub global_module: &'r mut Module,
    pub call_stack: &'r mut CallStack,
    pub data_stack: &'r mut DataStack,
    pub side_stack: &'r mut DataStack,
}

pub enum CoreBuiltinState {
    StepDone,
    FullyCompleted,
}

pub enum PlatformBuiltinState {
    Done,
    Sleeping,
}
