pub fn evaluate(tokens: Vec<String>, data_stack: &mut Vec<bool>) {
    for token in tokens {
        match token.as_str() {
            "true" => data_stack.push(true),
            "false" => data_stack.push(false),
            _ => {
                // Unexpected token. Eventually, it would be great to have some
                // real error reporting mechanism. For now, let's just make sure
                // the test won't pass.
                data_stack.clear();
                return;
            }
        }
    }
}
