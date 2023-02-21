use super::{
    evaluate, evaluator::FunctionNotFound, DataStack, Functions, Value,
};

pub type Builtin =
    fn(&Functions, &mut DataStack) -> Result<(), FunctionNotFound>;

pub fn get(name: &str) -> Option<Builtin> {
    let builtin = match name {
        "clone" => clone,
        "drop" => drop,
        "eval" => eval,
        "false" => false_,
        "if" => if_,
        "min" => min,
        "or" => or,
        "over" => over,
        "set_list" => set_list,
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
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = a.saturating_add(b);

    data_stack.push(x);

    Ok(())
}

fn clone(
    _: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    let value = data_stack.pop_any();

    data_stack.push(value.clone());
    data_stack.push(value);

    Ok(())
}

fn drop(
    _: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    data_stack.pop_any();

    Ok(())
}

fn eq(
    _: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    data_stack.push(a == b);

    Ok(())
}

fn eval(
    functions: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    let block = data_stack.pop_block();

    evaluate(&block, functions, data_stack)?;

    Ok(())
}

fn false_(
    _: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    data_stack.push(Value::Bool(false));
    Ok(())
}

fn if_(
    functions: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    let else_ = data_stack.pop_block();
    let then = data_stack.pop_block();
    let cond = data_stack.pop_bool();

    let block = if cond { then } else { else_ };
    evaluate(&block, functions, data_stack)?;

    Ok(())
}

fn min(
    _: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = u8::min(a, b);

    data_stack.push(x);

    Ok(())
}

fn or(
    _: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_bool();
    let a = data_stack.pop_bool();

    data_stack.push(a || b);

    Ok(())
}

fn over(
    _: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    let b = data_stack.pop_any();
    let a = data_stack.pop_any();

    data_stack.push(a.clone());
    data_stack.push(b);
    data_stack.push(a);

    Ok(())
}

fn set_list(
    _: &Functions,
    data_stack: &mut DataStack,
) -> Result<(), FunctionNotFound> {
    let value = data_stack.pop_any();
    let index = data_stack.pop_u8();
    let mut list = data_stack.pop_list();

    list[index as usize] = value;

    data_stack.push(list);

    Ok(())
}

fn sub(
    _: &Functions,
    data_stack: &mut DataStack,
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
) -> Result<(), FunctionNotFound> {
    data_stack.push(Value::Bool(true));
    Ok(())
}
