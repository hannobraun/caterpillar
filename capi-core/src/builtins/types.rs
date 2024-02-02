use crate::{
    pipeline::Module,
    platform::Platform,
    repr::eval::fragments::{FragmentId, Fragments},
    runtime::{
        call_stack::CallStack,
        data_stack::{DataStack, DataStackResult},
    },
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

#[derive(Debug)]
pub enum Builtin<P: Platform> {
    Core(CoreBuiltin),
    Platform(PlatformBuiltin<P>),
}

pub type CoreBuiltin =
    fn(step: usize, BuiltinContext) -> DataStackResult<CoreBuiltinState>;

// According to the warning, the bound is not enforced in the type alias. We
// still need it here, however, so we can refer to its associated types.
#[allow(type_alias_bounds)]
pub type PlatformBuiltin<P: Platform> =
    fn(
        BuiltinContext,
        &mut P::Context,
    ) -> DataStackResult<PlatformBuiltinState>;

pub enum CoreBuiltinState {
    StepDone,
    FullyCompleted,
}

pub enum PlatformBuiltinState {
    Done,
    Sleeping,
}
