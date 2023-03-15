pub fn evaluate(code: &str, data_stack: &mut Vec<bool>) {
    data_stack.push(code == "true");
}
