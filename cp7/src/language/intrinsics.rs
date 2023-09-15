use std::{collections::HashMap, thread, time::Duration};

use super::{
    repr::eval::value,
    runtime::{
        data_stack::DataStackResult, evaluator::Evaluator,
        functions::FunctionName,
    },
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

pub fn eval(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let block = evaluator.data_stack.pop_specific::<value::Block>()?;
    evaluator.call_stack.push(block.start);

    // `eval` doesn't need to consume the block, so it would be nice, if we
    // could it back on the stack. However, if we were to do that here, that
    // would happen *before* the block is evaluated, and hence the block would
    // have itself on the stack when it starts. This sounds like it could
    // possibly be useful, rarely in devious ways, but it certainly will just be
    // annoying in the common case.
    //
    // What we *could* do is add another frame to the call stack, which puts the
    // block back on the stack after the block itself returns. That would
    // require stack frames to be an enum that could either reference a fragment
    // or builtin code that does what we need.
    //
    // Not sure if it's worth it. Maybe if the need for this comes up in more
    // cases.

    Ok(())
}

pub fn fn_(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let body = evaluator.data_stack.pop_specific::<value::Block>()?;
    let name = evaluator.data_stack.pop_specific::<value::Symbol>()?;

    let name = FunctionName { value: name.0 };
    evaluator.functions.define(name, body);

    Ok(())
}

pub fn nop(_: &mut Evaluator) -> DataStackResult<()> {
    Ok(())
}

pub fn over(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let top = evaluator.data_stack.pop_any()?;
    let target = evaluator.data_stack.pop_any()?;

    evaluator.data_stack.push(target.clone());
    evaluator.data_stack.push(top);
    evaluator.data_stack.push(target);

    Ok(())
}

pub fn ping(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let channel = evaluator.data_stack.pop_specific::<value::Number>()?;
    *evaluator.context.channels.entry(channel.0).or_insert(0) += 1;
    Ok(())
}

pub fn print(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let value = evaluator.data_stack.pop_any()?;
    println!("{value}");
    evaluator.data_stack.push(value);
    Ok(())
}

pub fn swap(evaluator: &mut Evaluator) -> DataStackResult<()> {
    let b = evaluator.data_stack.pop_any()?;
    let a = evaluator.data_stack.pop_any()?;

    evaluator.data_stack.push(b);
    evaluator.data_stack.push(a);

    Ok(())
}
