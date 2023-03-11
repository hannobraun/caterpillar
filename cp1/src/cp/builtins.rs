use std::collections::VecDeque;

use super::{
    evaluate, evaluator::FunctionNotFound, Bindings, DataStack, Functions,
    Value,
};

pub type Builtin = fn(
    &Functions,
    &mut DataStack,
    &mut Bindings,
    bool,
) -> Result<(), FunctionNotFound>;

pub fn get(name: &str) -> Option<Builtin> {
    let builtin = match name {
        "and" => and,
        "bind" => bind,
        "clone" => clone,
        "drop" => drop,
        "eval" => eval,
        "false" => false_,
        "if" => if_,
        "list_get" => list_get,
        "list_set" => list_set,
        "min" => min,
        "not" => not,
        "or" => or,
        "over" => over,
        "print" => print,
        "rot" => rot,
        "shuffle" => shuffle,
        "swap" => swap,
        "true" => true_,
        "=" => eq,
        "+" => add,
        "-" => sub,
        _ => return None,
    };

    Some(builtin)
}

fn add(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = a.saturating_add(b);

    data_stack.push(x);

    Ok(())
}

fn and(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_bool();
    let a = data_stack.pop_bool();

    let x = a && b;

    data_stack.push(x);

    Ok(())
}

fn bind(
    _: &Functions,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let mut names = data_stack.pop_list();
    names.values.reverse();

    for name in names {
        let Value::Name(name) = name else {
            panic!("Expected name")
        };
        let value = data_stack.pop_any();

        bindings.create(name, value);
    }

    Ok(())
}

fn clone(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let value = data_stack.pop_any();

    data_stack.push(value.clone());
    data_stack.push(value);

    Ok(())
}

fn drop(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    data_stack.pop_any();

    Ok(())
}

fn eq(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    data_stack.push(a == b);

    Ok(())
}

fn eval(
    functions: &Functions,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    debug_output: bool,
) -> Result<(), FunctionNotFound> {
    let block = data_stack.pop_block();

    evaluate(
        &block.expressions,
        functions,
        data_stack,
        bindings,
        debug_output,
    )?;

    Ok(())
}

fn false_(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    data_stack.push(Value::Bool(false));
    Ok(())
}

fn if_(
    functions: &Functions,
    data_stack: &mut DataStack,
    bindings: &mut Bindings,
    debug_output: bool,
) -> Result<(), FunctionNotFound> {
    let else_ = data_stack.pop_block();
    let then = data_stack.pop_block();
    let cond = data_stack.pop_bool();

    let block = if cond { then } else { else_ };
    evaluate(
        &block.expressions,
        functions,
        data_stack,
        bindings,
        debug_output,
    )?;

    Ok(())
}

fn list_get(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let index = data_stack.pop_u8();
    let list = data_stack.pop_list();

    let value = list.values[index as usize].clone();

    data_stack.push(list);
    data_stack.push(value);

    Ok(())
}

fn list_set(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let value = data_stack.pop_any();
    let index = data_stack.pop_u8();
    let mut list = data_stack.pop_list();

    list.values[index as usize] = value;

    data_stack.push(list);

    Ok(())
}

fn min(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = u8::min(a, b);

    data_stack.push(x);

    Ok(())
}

fn not(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let v = data_stack.pop_bool();

    let x = !v;

    data_stack.push(x);

    Ok(())
}

fn or(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_bool();
    let a = data_stack.pop_bool();

    data_stack.push(a || b);

    Ok(())
}

fn over(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_any();
    let a = data_stack.pop_any();

    data_stack.push(a.clone());
    data_stack.push(b);
    data_stack.push(a);

    Ok(())
}

fn print(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let value = data_stack.pop_any();
    eprintln!("{value}");
    Ok(())
}

fn rot(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let c = data_stack.pop_any();
    let b = data_stack.pop_any();
    let a = data_stack.pop_any();

    data_stack.push(b);
    data_stack.push(c);
    data_stack.push(a);

    Ok(())
}

fn shuffle(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let indices = data_stack.pop_list().into_iter().map(|value| {
        let Value::U8(value) = value else {
            panic!("Expected `u8`")
        };
        value
    });

    let mut values = VecDeque::new();
    for _ in 0..indices.len() {
        values.push_front(data_stack.pop_any());
    }

    for index in indices {
        data_stack.push(values[index as usize].clone());
    }

    Ok(())
}

fn sub(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = a.saturating_sub(b);

    data_stack.push(x);

    Ok(())
}

fn swap(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_any();
    let a = data_stack.pop_any();

    data_stack.push(b);
    data_stack.push(a);

    Ok(())
}

fn true_(
    _: &Functions,
    data_stack: &mut DataStack,
    _: &mut Bindings,
    _: bool,
) -> Result<(), FunctionNotFound> {
    data_stack.push(Value::Bool(true));
    Ok(())
}
