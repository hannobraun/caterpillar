use crate::{
    pipeline::Module,
    repr::eval::{
        fragments::{FragmentId, Fragments},
        value::TypeError,
    },
    runtime::{
        call_stack::CallStack,
        data_stack::{DataStack, DataStackError},
    },
};

use super::core::ArrayIndexOutOfBounds;

pub trait Platform: Sized {
    type Context<'r>;
    type Error;

    fn builtin_fns() -> impl BuiltinFns<Self>;
}

pub trait BuiltinFns<P: Platform>:
    IntoIterator<Item = (BuiltinFn<P>, &'static str)>
{
}

impl<T, P: Platform> BuiltinFns<P> for T where
    T: IntoIterator<Item = (BuiltinFn<P>, &'static str)>
{
}

// According to the warning, the bound is not enforced in the type alias. We
// still need it here, however, so we can refer to its associated types.
#[allow(type_alias_bounds)]
pub type BuiltinFn<P: Platform> = fn(
    step: usize,
    // The lack of symmetry between the following two arguments lacks elegance.
    // The way `CoreContext` is designed is probably the more elegant way for a
    // type like this, so I think it makes sense to have platform contexts
    // designed the same way.
    core_context: CoreContext,
    platform_context: &mut P::Context<'_>,
) -> BuiltinFnResult<P::Error>;

pub type BuiltinFnResult<E> = Result<BuiltinFnState, BuiltinFnError<E>>;

#[derive(Debug, thiserror::Error)]
pub enum BuiltinFnError<T> {
    #[error("Error operating data stack")]
    DataStack(#[from] DataStackError),

    #[error(transparent)]
    ArrayIndexOutOfBounds(#[from] ArrayIndexOutOfBounds),

    #[error(transparent)]
    Type(#[from] TypeError),

    #[error("Platform-specific error from builtin function")]
    PlatformSpecific(T),
}

// I don't like how `CoreContext` looks here. Everything else here is nice and
// self-contained, but `CoreContext` depends on a whole lot of stuff.
//
// The problematic part is the dependency on stuff in `runtime`. Some of
// `runtime` depends back on this module. That muddles the dependency graph.
pub struct CoreContext<'r> {
    /// The fragment ID of the word that refers to this intrinsic or platform fn
    pub word: FragmentId,

    pub fragments: &'r mut Fragments,
    pub global_module: &'r mut Module,
    pub call_stack: &'r mut CallStack,
    pub data_stack: &'r mut DataStack,
    pub side_stack: &'r mut DataStack,
}

pub enum BuiltinFnState {
    Completed,
    Sleeping,
    Stepped,
}
