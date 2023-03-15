pub fn evaluate(tokens: Vec<String>, data_stack: &mut Vec<bool>) {
    for token in tokens {
        data_stack.push(token == "true");
    }
}
