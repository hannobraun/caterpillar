use crate::builtins::types::PlatformBuiltin;

pub trait Platform: Sized {
    type Context;

    fn builtins() -> impl PlatformBuiltins<Self>;
}

pub trait PlatformBuiltins<P: Platform>:
    IntoIterator<Item = (PlatformBuiltin<P>, &'static str)>
{
}

impl<T, P: Platform> PlatformBuiltins<P> for T where
    T: IntoIterator<Item = (PlatformBuiltin<P>, &'static str)>
{
}
