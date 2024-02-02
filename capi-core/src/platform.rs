use crate::builtins::types::PlatformBuiltin;

pub trait Platform {
    type Context;

    fn functions(
    ) -> impl IntoIterator<Item = (PlatformBuiltin<Self>, &'static str)>;
}
