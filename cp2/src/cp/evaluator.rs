pub fn evaluate(code: &str, data_stack: &mut Vec<bool>) {
    for word in code.split_whitespace() {
        data_stack.push(word == "true");
    }
}
