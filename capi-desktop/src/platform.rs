use std::{thread, time::Duration};

use capi_core::{value, DataStackResult, FunctionState, RuntimeContext};

pub struct Context {
    pub pixel_operations: Vec<[i64; 2]>,
}

pub fn pixel_set(
    runtime_context: RuntimeContext,
    platform_context: &mut Context,
) -> DataStackResult<FunctionState> {
    let (y, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;
    let (x, _) = runtime_context.data_stack.pop_specific::<value::Number>()?;

    platform_context.pixel_operations.push([x.0, y.0]);

    Ok(FunctionState::Done)
}

pub fn delay_ms(
    runtime_context: RuntimeContext,
    _: &mut Context,
) -> DataStackResult<FunctionState> {
    let (delay_ms, _) =
        runtime_context.data_stack.pop_specific::<value::Number>()?;
    thread::sleep(Duration::from_millis(delay_ms.0.try_into().unwrap()));
    Ok(FunctionState::Done)
}

pub fn print(
    runtime_context: RuntimeContext,
    _: &mut Context,
) -> DataStackResult<FunctionState> {
    let value = runtime_context.data_stack.pop_any()?;
    println!("{}", value.kind);
    runtime_context.data_stack.push(value);
    Ok(FunctionState::Done)
}
