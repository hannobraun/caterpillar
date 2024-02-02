use crate::{
    builtins::types::{CoreContext, PlatformBuiltinState},
    runtime::data_stack::DataStackResult,
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
pub type BuiltinFn<P: Platform> =
    fn(CoreContext, &mut P::Context) -> DataStackResult<PlatformBuiltinState>;
