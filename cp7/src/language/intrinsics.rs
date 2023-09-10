use std::{collections::HashMap, thread, time::Duration};

use super::{
    repr::eval::value,
    runtime::{data_stack::DataStackResult, evaluator::Evaluator},
};

#[derive(Debug)]
pub struct Context {
    pub channels: HashMap<i64, i64>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            channels: HashMap::new(),
        }
    }
}

pub fn add(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let b = evaluator.data_stack.pop_specific::<value::Number>()?;
    let a = evaluator.data_stack.pop_specific::<value::Number>()?;

    evaluator.data_stack.push(value::Number(a.0 + b.0));

    Ok(())
}

pub fn clone(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let value = evaluator.data_stack.pop_any()?;

    evaluator.data_stack.push(value.clone());
    evaluator.data_stack.push(value);

    Ok(())
}

pub fn delay_ms(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let delay_ms = evaluator.data_stack.pop_specific::<value::Number>()?;
    thread::sleep(Duration::from_millis(delay_ms.0.try_into().unwrap()));
    Ok(())
}

pub fn fn_(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let body = evaluator.data_stack.pop_specific::<value::Block>()?;
    let name = evaluator.data_stack.pop_specific::<value::Symbol>()?;

    evaluator.functions.define(name, body);

    Ok(())
}

pub fn ping(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let channel = evaluator.data_stack.pop_specific::<value::Number>()?;
    *evaluator.context.channels.entry(channel.0).or_insert(0) += 1;
    Ok(())
}

pub fn print_line(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let value = evaluator.data_stack.pop_any()?;
    println!("{value}");
    Ok(())
}
