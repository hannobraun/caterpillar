use std::time::Duration;

use capi_core::{DataStackResult, FunctionState, RuntimeContext};

#[derive(Default)]
pub struct Context {
    pub sleep_duration: Option<Duration>,
}

pub fn print(
    runtime_context: RuntimeContext,
    _: &mut Context,
) -> DataStackResult<FunctionState> {
    let value = runtime_context.data_stack.pop_any()?;
    tracing::info!("{}", value.kind);
    runtime_context.data_stack.push(value);
    Ok(FunctionState::Done)
}
