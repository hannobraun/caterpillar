use super::DataStack;

pub fn clone(data_stack: &mut DataStack) {
    let value = data_stack.pop_any();

    data_stack.push(value.clone());
    data_stack.push(value);
}

pub fn drop(data_stack: &mut DataStack) {
    data_stack.pop_any();
}

pub fn or(data_stack: &mut DataStack) {
    let b = data_stack.pop_bool();
    let a = data_stack.pop_bool();

    data_stack.push(a || b);
}

pub fn swap(data_stack: &mut DataStack) {
    let b = data_stack.pop_any();
    let a = data_stack.pop_any();

    data_stack.push(b);
    data_stack.push(a);
}

pub fn eq(data_stack: &mut DataStack) {
    let b = data_stack.pop_u8();
    let a = data_stack.pop_u8();

    data_stack.push(a == b);
}
