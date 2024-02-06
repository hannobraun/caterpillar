pub mod core;

mod platform;

pub use self::platform::{
    BuiltinFn, BuiltinFnError, BuiltinFnResult, BuiltinFnState, BuiltinFns,
    CoreContext, Platform,
};
