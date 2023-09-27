use std::time::Duration;

use capi_core::{value, DataStackResult, FunctionState, RuntimeContext};

#[derive(Default)]
pub struct Context {
    pub sleep_duration: Option<Duration>,
}

pub fn delay_ms(
    runtime_context: RuntimeContext,
    platform_context: &mut Context,
) -> DataStackResult<FunctionState> {
    let (delay_ms, _) =
        runtime_context.data_stack.pop_specific::<value::Number>()?;

    let delay_ms = delay_ms
        .0
        .try_into()
        .expect("Negative sleep duration is invalid");
    platform_context.sleep_duration = Some(Duration::from_millis(delay_ms));

    Ok(FunctionState::Sleeping)
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
