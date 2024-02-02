use crate::platform::{BuiltinFn, Platform};

use super::core::CorePlatform;

#[derive(Debug)]
pub enum Builtin<P: Platform> {
    Core(CoreBuiltin),
    Platform(BuiltinFn<P>),
}

pub type CoreBuiltin = BuiltinFn<CorePlatform>;
