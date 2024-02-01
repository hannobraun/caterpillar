use crate::runtime::namespaces::PlatformFunction;

pub trait Platform {
    type Context;

    fn functions(
    ) -> impl IntoIterator<Item = (PlatformFunction<Self::Context>, &'static str)>;
}
