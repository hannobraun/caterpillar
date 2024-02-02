use crate::builtins::types::PlatformBuiltin;

pub trait Platform: Sized {
    type Context;

    fn builtin_fns() -> impl BuiltinFns<Self>;
}

pub trait BuiltinFns<P: Platform>:
    IntoIterator<Item = (PlatformBuiltin<P>, &'static str)>
{
}

impl<T, P: Platform> BuiltinFns<P> for T where
    T: IntoIterator<Item = (PlatformBuiltin<P>, &'static str)>
{
}
