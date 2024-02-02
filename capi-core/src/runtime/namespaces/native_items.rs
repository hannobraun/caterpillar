use crate::builtins::types::{IntrinsicFunction, PlatformBuiltin};

#[derive(Debug)]
pub enum NativeFunction<C> {
    Intrinsic(IntrinsicFunction),
    Platform(PlatformBuiltin<C>),
}
