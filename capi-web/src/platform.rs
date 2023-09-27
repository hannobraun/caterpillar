pub fn print(
    context: capi_core::RuntimeContext,
    _: &mut (),
) -> capi_core::DataStackResult<capi_core::FunctionState> {
    let value = context.data_stack.pop_any()?;
    tracing::info!("{}", value.kind);
    context.data_stack.push(value);
    Ok(capi_core::FunctionState::Done)
}
