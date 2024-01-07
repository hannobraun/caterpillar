use crate::{
    runtime::{
        call_stack::StackFrame,
        namespaces::{
            IntrinsicFunction, IntrinsicFunctionState, RuntimeContext,
        },
    },
    value::{Value, ValuePayload},
};

use super::{
    repr::eval::value,
    runtime::{data_stack::DataStackResult, namespaces::FunctionName},
};

pub fn all() -> Vec<(IntrinsicFunction, &'static str)> {
    vec![
        (add, "+"),
        (and, "and"),
        (append, "append"),
        (array, "[]"),
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

fn add(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
            let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

            context.data_stack.push_bare(value::Number(a.0 + b.0));

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn and(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (b, _) = context.data_stack.pop_specific::<value::Bool>()?;
            let (a, _) = context.data_stack.pop_specific::<value::Bool>()?;

            context.data_stack.push_bare(value::Bool(a.0 && b.0));

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn append(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (block, _) =
                context.data_stack.pop_specific::<value::Block>()?;
            let (array, fragment) =
                context.data_stack.pop_specific::<value::Array>()?;

            context.side_stack.push(Value {
                payload: array.into(),
                fragment,
            });

            context.data_stack.mark();
            context.call_stack.push(StackFrame::Fragment {
                fragment_id: block.start,
            });

            Ok(IntrinsicFunctionState::StepDone)
        }
        1 => {
            let items = context
                .data_stack
                .drain_values_from_marker()
                .map(|value| value.payload);

            let (mut array, fragment) =
                context.side_stack.pop_specific::<value::Array>()?;

            array.0.extend(items);

            context.data_stack.push(Value {
                payload: array.into(),
                fragment,
            });

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn array(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            context.data_stack.push_bare(value::Array(Vec::new()));
            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn bind(
    step: usize,
    mut context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (symbols, _) =
                context.data_stack.pop_specific::<value::Array>()?;

            for symbol in symbols.0.into_iter().rev() {
                let symbol = symbol.expect::<value::Symbol>()?;
                let value = context.data_stack.pop_any()?;
                context.namespace.define_binding(symbol.0, value);
            }

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn clone(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let value = context.data_stack.pop_any()?;

            context.data_stack.push(value.clone());
            context.data_stack.push(value);

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn drop(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            context.data_stack.pop_any()?;
            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn each(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (block, _) =
                context.data_stack.pop_specific::<value::Block>()?;
            let (array, _) =
                context.data_stack.pop_specific::<value::Array>()?;

            // Turns out this doesn't work as intended. Here's what I wanted
            // this to do:
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
            // 1. All of the indices and array values are available on the stack
            //    all the time, increasing the chance for `block` mess
            //    everything up, if it accesses something that it shouldn't.
            // 2. Even if `block` works correctly, the array elements are
            //    accessed backwards.
            //
            // Unfortunately, solving this isn't trivial. We'd need to do one
            // operation of this loop, return control to the evaluator, then
            // make sure the evaluator returns control here afterwards, while
            // keeping our state here. There is currently no mechanism for
            // making this happen.
            //
            // We could solve this, if intrinsics were able to put arbitrary
            // code on the call stack somehow. There's also an issue with `eval`
            // (see comment there) that could be solved with the same solution,
            // so maybe that's worth doing now.
            //
            // Another option would be to not implement `each` as an intrinsic.
            // We can already do simple loops through recursion, so why not use
            // that to implement `each` in Caterpillar?
            for (i, value) in array.0.into_iter().enumerate() {
                context
                    .data_stack
                    .push_bare(value::Number(i.try_into().unwrap()));
                context.data_stack.push_bare(value);

                context.call_stack.push(StackFrame::Fragment {
                    fragment_id: block.start,
                });
            }

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn eq(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let b = context.data_stack.pop_any()?;
            let a = context.data_stack.pop_any()?;

            context
                .data_stack
                .push_bare(value::Bool(a.payload == b.payload));

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn eval(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (block, _) =
                context.data_stack.pop_specific::<value::Block>()?;
            context.call_stack.push(StackFrame::Fragment {
                fragment_id: block.start,
            });

            // `eval` doesn't need to consume the block, so it would be nice, if
            // we could put it back on the stack. However, if we were to do that
            // here, that would happen *before* the block is evaluated, and
            // hence the block would have itself on the stack when it starts.
            // This sounds like it could possibly be useful, rarely and in
            // devious ways, but it certainly will just be annoying in the
            // common case.
            //
            // What we *could* do is add another frame to the call stack, which
            // puts the block back on the stack after the block itself returns.
            // That would require stack frames to be an enum that could either
            // reference a fragment or builtin code that does what we need.
            //
            // Not sure if it's worth it. Maybe if the need for this comes up in
            // more cases.

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn false_(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            context.data_stack.push_bare(value::Bool(false));
            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn fn_(
    step: usize,
    mut context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (body, _) =
                context.data_stack.pop_specific::<value::Block>()?;
            let (name, name_fragment) =
                context.data_stack.pop_specific::<value::Symbol>()?;

            let name = FunctionName {
                value: name.0,
                fragment: name_fragment,
            };

            context.namespace.define_function(name, body);

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn get(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (index, _) =
                context.data_stack.pop_specific::<value::Number>()?;
            let (array, fragment) =
                context.data_stack.pop_specific::<value::Array>()?;

            let value = array.0[index.0 as usize].clone();

            context.data_stack.push(Value {
                payload: ValuePayload::Array(array.0),
                fragment,
            });
            context.data_stack.push_bare(value);

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn gt(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
            let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

            context.data_stack.push_bare(value::Bool(a.0 > b.0));

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn if_(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (else_, _) =
                context.data_stack.pop_specific::<value::Block>()?;
            let (then, _) =
                context.data_stack.pop_specific::<value::Block>()?;
            let (condition, _) =
                context.data_stack.pop_specific::<value::Bool>()?;

            let start = if condition.0 { then.start } else { else_.start };
            context
                .call_stack
                .push(StackFrame::Fragment { fragment_id: start });

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn len(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (array, fragment) =
                context.data_stack.pop_specific::<value::Array>()?;

            let len: i64 = array.0.len().try_into().unwrap();

            context.data_stack.push(Value {
                payload: array.into(),
                fragment,
            });
            context.data_stack.push_bare(ValuePayload::Number(len));

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn nop(
    step: usize,
    _: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => Ok(IntrinsicFunctionState::StepDone),
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn not(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (a, _) = context.data_stack.pop_specific::<value::Bool>()?;

            context.data_stack.push_bare(value::Bool(!a.0));

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn over(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let top = context.data_stack.pop_any()?;
            let target = context.data_stack.pop_any()?;

            context.data_stack.push(target.clone());
            context.data_stack.push(top);
            context.data_stack.push(target);

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn set(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let value = context.data_stack.pop_any()?;
            let (index, _) =
                context.data_stack.pop_specific::<value::Number>()?;
            let (mut array, _) =
                context.data_stack.pop_specific::<value::Array>()?;

            array.0[index.0 as usize] = value.payload;

            context.data_stack.push_bare(array);

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn sub(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
            let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

            context.data_stack.push_bare(value::Number(a.0 - b.0));

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn swap(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let b = context.data_stack.pop_any()?;
            let a = context.data_stack.pop_any()?;

            context.data_stack.push(b);
            context.data_stack.push(a);

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn test(
    step: usize,
    mut context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (body, _) =
                context.data_stack.pop_specific::<value::Block>()?;
            let (name, name_fragment) =
                context.data_stack.pop_specific::<value::Text>()?;

            let name = FunctionName {
                value: name.0,
                fragment: name_fragment,
            };

            context.namespace.define_test(name, body);

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn true_(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            context.data_stack.push_bare(value::Bool(true));
            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}

fn unwrap(
    step: usize,
    context: RuntimeContext,
) -> DataStackResult<IntrinsicFunctionState> {
    match step {
        0 => {
            let (array, _) =
                context.data_stack.pop_specific::<value::Array>()?;

            for item in array.0 {
                context.data_stack.push_bare(item);
            }

            Ok(IntrinsicFunctionState::StepDone)
        }
        _ => Ok(IntrinsicFunctionState::FullyCompleted),
    }
}
