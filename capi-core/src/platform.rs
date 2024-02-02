use crate::builtins::types::PlatformBuiltin;

pub trait Platform {
    type Context;

    fn builtins(
    ) -> impl IntoIterator<Item = (PlatformBuiltin<Self>, &'static str)>;
}
