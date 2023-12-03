use crate::{
    runtime::modules::{IntrinsicFunction, RuntimeContext},
    value::{Value, ValuePayload},
};

use super::{
    repr::eval::value,
    runtime::{data_stack::DataStackResult, modules::FunctionName},
};

pub fn all() -> Vec<(IntrinsicFunction, &'static str)> {
    vec![
        (add, "+"),
        (and, "and"),
        (bind, "bind"),
        (clone, "clone"),
        (drop, "drop"),
        (each, "each"),
        (eq, "="),
        (eval, "eval"),
        (false_, "false"),
        (fn_, "fn"),
        (get, "get"),
        (gt, ">"),
        (if_, "if"),
        (len, "len"),
        (nop, "nop"),
        (not, "not"),
        (over, "over"),
        (set, "set"),
        (sub, "-"),
        (swap, "swap"),
        (test, "test"),
        (true_, "true"),
        (unwrap, "unwrap"),
    ]
}

fn add(context: RuntimeContext) -> DataStackResult<()> {
    let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
    let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

    context.data_stack.push_bare(value::Number(a.0 + b.0));

    Ok(())
}

fn and(context: RuntimeContext) -> DataStackResult<()> {
    let (b, _) = context.data_stack.pop_specific::<value::Bool>()?;
    let (a, _) = context.data_stack.pop_specific::<value::Bool>()?;

    context.data_stack.push_bare(value::Bool(a.0 && b.0));

    Ok(())
}

fn bind(mut context: RuntimeContext) -> DataStackResult<()> {
    let (symbols, _) = context.data_stack.pop_specific::<value::Array>()?;

    for symbol in symbols.0.into_iter().rev() {
        let symbol = symbol.expect::<value::Symbol>()?;
        let value = context.data_stack.pop_any()?;
        context.namespace.define_binding(symbol.0, value);
    }

    Ok(())
}

fn clone(context: RuntimeContext) -> DataStackResult<()> {
    let value = context.data_stack.pop_any()?;

    context.data_stack.push(value.clone());
    context.data_stack.push(value);

    Ok(())
}

fn drop(context: RuntimeContext) -> DataStackResult<()> {
    context.data_stack.pop_any()?;
    Ok(())
}

fn each(context: RuntimeContext) -> DataStackResult<()> {
    let (block, _) = context.data_stack.pop_specific::<value::Block>()?;
    let (array, _) = context.data_stack.pop_specific::<value::Array>()?;

    // Turns out this doesn't work as intended. Here's what I wanted this to do:
    //
    // 1. Put `0` and first array value on data stack.
    // 2. Execute `block`.
    // 3. Put `1` and second array value on data stack.
    // 4. Execute `block`.
    // 5. ...
    //
    // Here's what it actually does:
    //
    // 1. Put `0` and first array value on data stack.
    // 2. Put `1` and second array value on data stack.
    // 3. ...
    // 4. Execute block.
    // 5. Execute block.
    // 6. ...
    //
    // There are two specific problems here:
    //
    // 1. All of the indices and array values are available on the stack all the
    //    time, increasing the change for `block` mess everything up, if it
    //    accesses something that it shouldn't.
    // 2. Even if `block` works correctly, the array elements are accessed
    //    backwards.
    //
    // Unfortunately, solving this isn't trivial. We'd need to do one operation
    // of this loop, return control to the evaluator, then make sure the
    // evaluator returns control here afterwards, while keeping our state here.
    // There is currently no mechanism for making this happen.
    //
    // We could solve this, if intrinsics were able to put arbitrary code on the
    // call stack somehow. There's also an issue with `eval` (see comment there)
    // that could be solved with the same solution, so maybe that's worth doing
    // now.
    //
    // Another option would be to not implement `each` as an intrinsic. We can
    // already do simple loop through recursion, so why not use that to
    // implement `each` in Caterpillar?
    for (i, value) in array.0.into_iter().enumerate() {
        context
            .data_stack
            .push_bare(value::Number(i.try_into().unwrap()));
        context.data_stack.push_bare(value);

        context.call_stack.push(block.start);
    }

    Ok(())
}

fn eq(context: RuntimeContext) -> DataStackResult<()> {
    let b = context.data_stack.pop_any()?;
    let a = context.data_stack.pop_any()?;

    context
        .data_stack
        .push_bare(value::Bool(a.payload == b.payload));

    Ok(())
}

fn eval(context: RuntimeContext) -> DataStackResult<()> {
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

fn false_(context: RuntimeContext) -> DataStackResult<()> {
    context.data_stack.push_bare(value::Bool(false));
    Ok(())
}

fn fn_(mut context: RuntimeContext) -> DataStackResult<()> {
    let (body, _) = context.data_stack.pop_specific::<value::Block>()?;
    let (name, name_fragment) =
        context.data_stack.pop_specific::<value::Symbol>()?;

    let name = FunctionName {
        value: name.0,
        fragment: name_fragment,
    };

    context.namespace.define_function(name, body);

    Ok(())
}

fn get(context: RuntimeContext) -> DataStackResult<()> {
    let (index, _) = context.data_stack.pop_specific::<value::Number>()?;
    let (array, fragment) =
        context.data_stack.pop_specific::<value::Array>()?;

    let value = array.0[index.0 as usize].clone();

    context.data_stack.push(Value {
        payload: ValuePayload::Array(array.0),
        fragment,
    });
    context.data_stack.push_bare(value);

    Ok(())
}

fn gt(context: RuntimeContext) -> DataStackResult<()> {
    let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
    let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

    context.data_stack.push_bare(value::Bool(a.0 > b.0));

    Ok(())
}

fn if_(context: RuntimeContext) -> DataStackResult<()> {
    let (else_, _) = context.data_stack.pop_specific::<value::Block>()?;
    let (then, _) = context.data_stack.pop_specific::<value::Block>()?;
    let (condition, _) = context.data_stack.pop_specific::<value::Bool>()?;

    let start = if condition.0 { then.start } else { else_.start };
    context.call_stack.push(start);

    Ok(())
}

fn len(context: RuntimeContext) -> DataStackResult<()> {
    let (array, fragment) =
        context.data_stack.pop_specific::<value::Array>()?;

    let len: i64 = array.0.len().try_into().unwrap();

    context.data_stack.push(Value {
        payload: array.into(),
        fragment,
    });
    context.data_stack.push_bare(ValuePayload::Number(len));

    Ok(())
}

fn nop(_: RuntimeContext) -> DataStackResult<()> {
    Ok(())
}

fn not(context: RuntimeContext) -> DataStackResult<()> {
    let (a, _) = context.data_stack.pop_specific::<value::Bool>()?;

    context.data_stack.push_bare(value::Bool(!a.0));

    Ok(())
}

fn over(context: RuntimeContext) -> DataStackResult<()> {
    let top = context.data_stack.pop_any()?;
    let target = context.data_stack.pop_any()?;

    context.data_stack.push(target.clone());
    context.data_stack.push(top);
    context.data_stack.push(target);

    Ok(())
}

fn set(context: RuntimeContext) -> DataStackResult<()> {
    let value = context.data_stack.pop_any()?;
    let (index, _) = context.data_stack.pop_specific::<value::Number>()?;
    let (mut array, _) = context.data_stack.pop_specific::<value::Array>()?;

    array.0[index.0 as usize] = value.payload;

    context.data_stack.push_bare(array);

    Ok(())
}

fn sub(context: RuntimeContext) -> DataStackResult<()> {
    let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
    let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

    context.data_stack.push_bare(value::Number(a.0 - b.0));

    Ok(())
}

fn swap(context: RuntimeContext) -> DataStackResult<()> {
    let b = context.data_stack.pop_any()?;
    let a = context.data_stack.pop_any()?;

    context.data_stack.push(b);
    context.data_stack.push(a);

    Ok(())
}

fn test(mut context: RuntimeContext) -> DataStackResult<()> {
    let (body, _) = context.data_stack.pop_specific::<value::Block>()?;
    let (name, name_fragment) =
        context.data_stack.pop_specific::<value::Text>()?;

    let name = FunctionName {
        value: name.0,
        fragment: name_fragment,
    };

    context.namespace.define_test(name, body);

    Ok(())
}

fn true_(context: RuntimeContext) -> DataStackResult<()> {
    context.data_stack.push_bare(value::Bool(true));
    Ok(())
}

fn unwrap(context: RuntimeContext) -> DataStackResult<()> {
    let (array, _) = context.data_stack.pop_specific::<value::Array>()?;

    for item in array.0 {
        context.data_stack.push_bare(item);
    }

    Ok(())
}
