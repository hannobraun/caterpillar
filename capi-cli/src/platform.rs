use std::{thread, time::Duration};

use capi_core::{value, DataStackResult, FunctionState, RuntimeContext};

pub fn delay_ms(
    context: RuntimeContext,
    _: &mut (),
) -> DataStackResult<FunctionState> {
    let (delay_ms, _) = context.data_stack.pop_specific::<value::Number>()?;
    thread::sleep(Duration::from_millis(delay_ms.0.try_into().unwrap()));
    Ok(FunctionState::Resume)
}

pub fn print(
    context: RuntimeContext,
    _: &mut (),
) -> DataStackResult<FunctionState> {
    let value = context.data_stack.pop_any()?;
    println!("{}", value.kind);
    context.data_stack.push(value);
    Ok(FunctionState::Resume)
}
