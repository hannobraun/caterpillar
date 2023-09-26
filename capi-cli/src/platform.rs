use std::{thread, time::Duration};

use capi_core::{value, DataStackResult, RuntimeContext};

pub fn delay_ms(context: RuntimeContext, _: &mut ()) -> DataStackResult<()> {
    let (delay_ms, _) = context.data_stack.pop_specific::<value::Number>()?;
    thread::sleep(Duration::from_millis(delay_ms.0.try_into().unwrap()));
    Ok(())
}

pub fn print(context: RuntimeContext, _: &mut ()) -> DataStackResult<()> {
    let value = context.data_stack.pop_any()?;
    println!("{}", value.kind);
    context.data_stack.push(value);
    Ok(())
}
