use crate::builtins::types::{CoreBuiltin, PlatformBuiltin};

#[derive(Debug)]
pub enum NativeFunction<C> {
    Intrinsic(CoreBuiltin),
    Platform(PlatformBuiltin<C>),
}
