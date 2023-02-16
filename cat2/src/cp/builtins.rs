use super::{DataStack, Functions};

pub type Builtin = fn(&Functions, &mut DataStack);

pub fn get(name: &str) -> Option<Builtin> {
    let builtin = match name {
        "clone" => clone,
        "drop" => drop,
        "min" => min,
        "or" => or,
        "swap" => swap,
        "=" => eq,
        "+" => add,
        "-" => sub,
        _ => return None,
    };

    Some(builtin)
}

fn add(_: &Functions, data_stack: &mut DataStack) {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = a.saturating_add(b);

    data_stack.push(x);
}

fn clone(_: &Functions, data_stack: &mut DataStack) {
    let value = data_stack.pop_any();

    data_stack.push(value.clone());
    data_stack.push(value);
}

fn drop(_: &Functions, data_stack: &mut DataStack) {
    data_stack.pop_any();
}

fn eq(_: &Functions, data_stack: &mut DataStack) {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    data_stack.push(a == b);
}

fn min(_: &Functions, data_stack: &mut DataStack) {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = u8::min(a, b);

    data_stack.push(x);
}

fn or(_: &Functions, data_stack: &mut DataStack) {
    let b = data_stack.pop_bool();
    let a = data_stack.pop_bool();

    data_stack.push(a || b);
}

fn sub(_: &Functions, data_stack: &mut DataStack) {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = a.saturating_sub(b);

    data_stack.push(x);
}

fn swap(_: &Functions, data_stack: &mut DataStack) {
    let b = data_stack.pop_any();
    let a = data_stack.pop_any();

    data_stack.push(b);
    data_stack.push(a);
}
