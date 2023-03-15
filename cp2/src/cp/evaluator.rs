pub fn evaluate(tokens: Vec<String>, data_stack: &mut Vec<bool>) {
    for token in tokens {
        match token.as_str() {
            "true" => data_stack.push(true),
            _ => data_stack.push(false),
        }
    }
}
