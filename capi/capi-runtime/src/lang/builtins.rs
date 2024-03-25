use super::data_stack::DataStack;

pub fn add(data_stack: &mut DataStack) {
    let b = data_stack.pop();
    let a = data_stack.pop();

    let c = a + b;

    data_stack.push(c);
}
