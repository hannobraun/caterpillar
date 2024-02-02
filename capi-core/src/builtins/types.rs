use crate::platform::{BuiltinFn, Platform};

use super::core::CorePlatform;

#[derive(Debug)]
pub enum Builtin<P: Platform> {
    Core(BuiltinFn<CorePlatform>),
    Platform(BuiltinFn<P>),
}
