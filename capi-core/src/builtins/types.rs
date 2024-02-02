use crate::{
    pipeline::Module,
    platform::{BuiltinFn, Platform},
    repr::eval::fragments::{FragmentId, Fragments},
    runtime::{
        call_stack::CallStack,
        data_stack::{DataStack, DataStackResult},
    },
};

pub struct CoreContext<'r> {
    /// The fragment ID of the word that refers to this intrinsic or platform fn
    pub word: FragmentId,

    pub fragments: &'r mut Fragments,
    pub global_module: &'r mut Module,
    pub call_stack: &'r mut CallStack,
    pub data_stack: &'r mut DataStack,
    pub side_stack: &'r mut DataStack,
}

#[derive(Debug)]
pub enum Builtin<P: Platform> {
    Core(CoreBuiltin),
    Platform(BuiltinFn<P>),
}

pub type CoreBuiltin =
    fn(step: usize, CoreContext) -> DataStackResult<CoreBuiltinState>;

pub enum CoreBuiltinState {
    StepDone,
    FullyCompleted,
}

pub enum PlatformBuiltinState {
    Done,
    Sleeping,
}
