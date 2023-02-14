use super::DataStack;

pub fn get(name: &str) -> Option<fn(&mut DataStack)> {
    let builtin = match name {
        "clone" => clone,
        "drop" => drop,
        "or" => or,
        "swap" => swap,
        "=" => eq,
        "+" => add,
        "-" => sub,
        _ => return None,
    };

    Some(builtin)
}

fn add(data_stack: &mut DataStack) {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = a.saturating_add(b);

    data_stack.push(x);
}

fn clone(data_stack: &mut DataStack) {
    let value = data_stack.pop_any();

    data_stack.push(value.clone());
    data_stack.push(value);
}

fn drop(data_stack: &mut DataStack) {
    data_stack.pop_any();
}

fn eq(data_stack: &mut DataStack) {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    data_stack.push(a == b);
}

fn or(data_stack: &mut DataStack) {
    let b = data_stack.pop_bool();
    let a = data_stack.pop_bool();

    data_stack.push(a || b);
}

fn sub(data_stack: &mut DataStack) {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    let x = a.saturating_sub(b);

    data_stack.push(x);
}

fn swap(data_stack: &mut DataStack) {
    let b = data_stack.pop_any();
    let a = data_stack.pop_any();

    data_stack.push(b);
    data_stack.push(a);
}
