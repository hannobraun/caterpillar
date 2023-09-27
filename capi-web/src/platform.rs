use capi_core::{DataStackResult, FunctionState, RuntimeContext};

pub fn print(
    context: RuntimeContext,
    _: &mut (),
) -> DataStackResult<FunctionState> {
    let value = context.data_stack.pop_any()?;
    tracing::info!("{}", value.kind);
    context.data_stack.push(value);
    Ok(FunctionState::Done)
}
