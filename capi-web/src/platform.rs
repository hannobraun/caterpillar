use std::time::Duration;

use capi_core::{DataStackResult, FunctionState, RuntimeContext};

#[derive(Default)]
pub struct Context {
    pub sleep_duration: Option<Duration>,
}

pub fn print(
    context: RuntimeContext,
    _: &mut Context,
) -> DataStackResult<FunctionState> {
    let value = context.data_stack.pop_any()?;
    tracing::info!("{}", value.kind);
    context.data_stack.push(value);
    Ok(FunctionState::Done)
}
