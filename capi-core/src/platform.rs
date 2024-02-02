use crate::{
    pipeline::Module,
    repr::eval::fragments::{FragmentId, Fragments},
    runtime::{
        call_stack::CallStack,
        data_stack::{DataStack, DataStackResult},
    },
};

pub trait Platform: Sized {
    type Context;

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
    // The way `CoreContext` is designed if probably the more elegant way for a
    // type like this, so I think it makes sense to have platform contexts
    // designed the same way.
    core_context: CoreContext,
    platform_context: &mut P::Context,
) -> DataStackResult<BuiltinFnState>;

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
