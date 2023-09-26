use crate::runtime::functions::RuntimeContext;

use super::{
    repr::eval::value,
    runtime::{data_stack::DataStackResult, functions::FunctionName},
};

pub fn add(context: RuntimeContext) -> DataStackResult<()> {
    let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
    let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

    context.data_stack.push_bare(value::Number(a.0 + b.0));

    Ok(())
}

pub fn clone(evaluator: RuntimeContext) -> DataStackResult<()> {
    let value = evaluator.data_stack.pop_any()?;

    evaluator.data_stack.push(value.clone());
    evaluator.data_stack.push(value);

    Ok(())
}

pub fn eval(evaluator: RuntimeContext) -> DataStackResult<()> {
    let (block, _) = evaluator.data_stack.pop_specific::<value::Block>()?;
    evaluator.call_stack.push(block.start);

    // `eval` doesn't need to consume the block, so it would be nice, if we
    // could put it back on the stack. However, if we were to do that here, that
    // would happen *before* the block is evaluated, and hence the block would
    // have itself on the stack when it starts. This sounds like it could
    // possibly be useful, rarely and in devious ways, but it certainly will
    // just be annoying in the common case.
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

pub fn fn_(mut evaluator: RuntimeContext) -> DataStackResult<()> {
    let (body, _) = evaluator.data_stack.pop_specific::<value::Block>()?;
    let (name, name_fragment) =
        evaluator.data_stack.pop_specific::<value::Symbol>()?;

    let name = FunctionName {
        value: name.0,
        fragment: name_fragment,
    };
    evaluator.functions.define(name, body);

    Ok(())
}

pub fn nop(_: RuntimeContext) -> DataStackResult<()> {
    Ok(())
}

pub fn over(evaluator: RuntimeContext) -> DataStackResult<()> {
    let top = evaluator.data_stack.pop_any()?;
    let target = evaluator.data_stack.pop_any()?;

    evaluator.data_stack.push(target.clone());
    evaluator.data_stack.push(top);
    evaluator.data_stack.push(target);

    Ok(())
}

pub fn swap(evaluator: RuntimeContext) -> DataStackResult<()> {
    let b = evaluator.data_stack.pop_any()?;
    let a = evaluator.data_stack.pop_any()?;

    evaluator.data_stack.push(b);
    evaluator.data_stack.push(a);

    Ok(())
}
