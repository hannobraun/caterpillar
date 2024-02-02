pub mod core;

mod platform;

pub use self::platform::{
    BuiltinFn, BuiltinFnState, BuiltinFns, CoreContext, Platform,
};
