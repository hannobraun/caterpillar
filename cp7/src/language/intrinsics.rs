use std::{thread, time::Duration};

use super::{
    repr::eval::value,
    runtime::{
        data_stack::{DataStack, DataStackResult},
        functions::Functions,
    },
};

#[derive(Debug)]
pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn add(
    _: &mut Context,
    _: &mut Functions,
    data_stack: &mut DataStack,
) -> DataStackResult<()> {
    let b = data_stack.pop_specific::<value::Number>()?;
    let a = data_stack.pop_specific::<value::Number>()?;

    data_stack.push(value::Number(a.0 + b.0));

    Ok(())
}

pub fn clone(
    _: &mut Context,
    _: &mut Functions,
    data_stack: &mut DataStack,
) -> DataStackResult<()> {
    let value = data_stack.pop_any()?;

    data_stack.push(value.clone());
    data_stack.push(value);

    Ok(())
}

pub fn delay_ms(
    _: &mut Context,
    _: &mut Functions,
    data_stack: &mut DataStack,
) -> DataStackResult<()> {
    let delay_ms = data_stack.pop_specific::<value::Number>()?;
    thread::sleep(Duration::from_millis(delay_ms.0.try_into().unwrap()));
    Ok(())
}

pub fn fn_(
    _: &mut Context,
    functions: &mut Functions,
    data_stack: &mut DataStack,
) -> DataStackResult<()> {
    let body = data_stack.pop_specific::<value::Block>()?;
    let name = data_stack.pop_specific::<value::Symbol>()?;

    functions.define(name, body);

    Ok(())
}

pub fn print_line(
    _: &mut Context,
    _: &mut Functions,
    data_stack: &mut DataStack,
) -> DataStackResult<()> {
    let value = data_stack.pop_any()?;
    println!("{value}");
    Ok(())
}
