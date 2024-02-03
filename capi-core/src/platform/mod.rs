pub mod core;

mod platform;

pub use self::platform::{
    BuiltinFn, BuiltinFnResult, BuiltinFnState, BuiltinFns, CoreContext,
    Platform,
};
