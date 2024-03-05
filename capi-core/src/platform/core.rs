use crate::{
    platform::{BuiltinFn, BuiltinFnState, BuiltinFns, CoreContext, Platform},
    repr::eval::value::{self, Value, ValuePayload},
    runtime::call_stack::StackFrame,
};

use super::BuiltinFnResult;

pub struct CorePlatform;

impl Platform for CorePlatform {
    type Context<'r> = ();
    type Error = ();

    fn builtin_fns() -> impl BuiltinFns<Self> {
        [
            (add as BuiltinFn<Self>, "+"),
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
            (true_, "true"),
            (unwrap, "unwrap"),
        ]
    }
}

fn add(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
            let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

            context.data_stack.push_bare(value::Number(a.0 + b.0));

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn and(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (b, _) = context.data_stack.pop_specific::<value::Bool>()?;
            let (a, _) = context.data_stack.pop_specific::<value::Bool>()?;

            context.data_stack.push_bare(value::Bool(a.0 && b.0));

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn append(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
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

            Ok(BuiltinFnState::Stepped)
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

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn array(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            context.data_stack.push_bare(value::Array(Vec::new()));
            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn bind(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (symbols, _) =
                context.data_stack.pop_specific::<value::Array>()?;

            for symbol in symbols.0.into_iter().rev() {
                let symbol = symbol.expect::<value::Symbol>()?;
                let value = context.data_stack.pop_any()?;
                context.global_module.define_binding(symbol.0, value);
            }

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn clone(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let value = context.data_stack.pop_any()?;

            context.data_stack.push(value.clone());
            context.data_stack.push(value);

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn drop(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            context.data_stack.pop_any()?;
            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn each(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
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
            //    all the time, increasing the chance for `block` to mess
            //    everything up, if it accesses something that it shouldn't.
            // 2. Even if `block` works correctly, the array elements are
            //    accessed backwards.
            //
            // We could solve this by running a single loop iteration, returning
            // control to the evaluator, then make sure the evaluator returns
            // control here afterwards, while keeping our state here. This
            // should now be possible using steps and the side stack.
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

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn eq(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let b = context.data_stack.pop_any()?;
            let a = context.data_stack.pop_any()?;

            context
                .data_stack
                .push_bare(value::Bool(a.payload == b.payload));

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn eval(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (block, _) =
                context.data_stack.pop_specific::<value::Block>()?;
            context.call_stack.push(StackFrame::Fragment {
                fragment_id: block.start,
            });

            // `eval` doesn't need to consume the block, so it would be nice, if
            // we could put it back on the stack. All infrastructure we need for
            // that exists now: Just put it on the side stack, put it back on
            // the main stack in the next step.

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn false_(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            context.data_stack.push_bare(value::Bool(false));
            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn get(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (index, _) =
                context.data_stack.pop_specific::<value::Number>()?;
            let (array, fragment) =
                context.data_stack.pop_specific::<value::Array>()?;

            let index = index.0 as usize;
            let value =
                array.0.get(index).cloned().ok_or(ArrayIndexOutOfBounds {
                    len: array.0.len(),
                    index,
                })?;

            context.data_stack.push(Value {
                payload: ValuePayload::Array(array.0),
                fragment,
            });
            context.data_stack.push_bare(value);

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn gt(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
            let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

            context.data_stack.push_bare(value::Bool(a.0 > b.0));

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn if_(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
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

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn len(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
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

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn nop(
    step: usize,
    _: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => Ok(BuiltinFnState::Stepped),
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn not(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (a, _) = context.data_stack.pop_specific::<value::Bool>()?;

            context.data_stack.push_bare(value::Bool(!a.0));

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn over(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let top = context.data_stack.pop_any()?;
            let target = context.data_stack.pop_any()?;

            context.data_stack.push(target.clone());
            context.data_stack.push(top);
            context.data_stack.push(target);

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn set(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let value = context.data_stack.pop_any()?;
            let (index, _) =
                context.data_stack.pop_specific::<value::Number>()?;
            let (mut array, _) =
                context.data_stack.pop_specific::<value::Array>()?;

            array.0[index.0 as usize] = value.payload;

            context.data_stack.push_bare(array);

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn sub(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (b, _) = context.data_stack.pop_specific::<value::Number>()?;
            let (a, _) = context.data_stack.pop_specific::<value::Number>()?;

            context.data_stack.push_bare(value::Number(a.0 - b.0));

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn swap(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let b = context.data_stack.pop_any()?;
            let a = context.data_stack.pop_any()?;

            context.data_stack.push(b);
            context.data_stack.push(a);

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn true_(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            context.data_stack.push_bare(value::Bool(true));
            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

fn unwrap(
    step: usize,
    context: CoreContext,
    _platform_context: &mut (),
) -> BuiltinFnResult<()> {
    match step {
        0 => {
            let (array, _) =
                context.data_stack.pop_specific::<value::Array>()?;

            for item in array.0 {
                context.data_stack.push_bare(item);
            }

            Ok(BuiltinFnState::Stepped)
        }
        _ => Ok(BuiltinFnState::Completed),
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Error index out of bounds; len `{len}`, index `{index}`")]
pub struct ArrayIndexOutOfBounds {
    pub len: usize,
    pub index: usize,
}
