pub fn evaluate(tokens: Vec<String>, data_stack: &mut Vec<bool>) {
    for word in tokens {
        data_stack.push(word == "true");
    }
}
