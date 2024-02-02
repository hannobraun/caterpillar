use crate::{
    platform::{BuiltinFn, CoreContext, Platform},
    runtime::data_stack::DataStackResult,
};

#[derive(Debug)]
pub enum Builtin<P: Platform> {
    Core(CoreBuiltin),
    Platform(BuiltinFn<P>),
}

pub type CoreBuiltin =
    fn(step: usize, CoreContext) -> DataStackResult<CoreBuiltinState>;

pub enum CoreBuiltinState {
    StepDone,
    FullyCompleted,
}
