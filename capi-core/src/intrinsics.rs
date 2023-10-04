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

pub fn clone(context: RuntimeContext) -> DataStackResult<()> {
    let value = context.data_stack.pop_any()?;

    context.data_stack.push(value.clone());
    context.data_stack.push(value);

    Ok(())
}

pub fn eval(context: RuntimeContext) -> DataStackResult<()> {
    let (block, _) = context.data_stack.pop_specific::<value::Block>()?;
    context.call_stack.push(block.start);

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

pub fn fn_(mut context: RuntimeContext) -> DataStackResult<()> {
    let (body, _) = context.data_stack.pop_specific::<value::Block>()?;
    let (name, name_fragment) =
        context.data_stack.pop_specific::<value::Symbol>()?;

    let name = FunctionName {
        value: name.0,
        fragment: name_fragment,
    };
    let is_test = false;

    context.functions.define(name, body, is_test);

    Ok(())
}

pub fn nop(_: RuntimeContext) -> DataStackResult<()> {
    Ok(())
}

pub fn over(context: RuntimeContext) -> DataStackResult<()> {
    let top = context.data_stack.pop_any()?;
    let target = context.data_stack.pop_any()?;

    context.data_stack.push(target.clone());
    context.data_stack.push(top);
    context.data_stack.push(target);

    Ok(())
}

pub fn swap(context: RuntimeContext) -> DataStackResult<()> {
    let b = context.data_stack.pop_any()?;
    let a = context.data_stack.pop_any()?;

    context.data_stack.push(b);
    context.data_stack.push(a);

    Ok(())
}
