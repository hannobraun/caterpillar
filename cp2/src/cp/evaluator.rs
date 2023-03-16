pub fn evaluate(
    tokens: Vec<String>,
    data_stack: &mut Vec<bool>,
) -> Result<(), Error> {
    for token in tokens {
        match token.as_str() {
            "true" => data_stack.push(true),
            "false" => data_stack.push(false),
            "not" => {
                // We should have some mechanism that reports an error, if the
                // stack is empty.
                let x = data_stack.pop().unwrap();
                data_stack.push(!x);
            }
            _ => {
                return Err(Error::UnexpectedToken(token));
            }
        }
    }

    Ok(())
}

pub enum Error {
    UnexpectedToken(String),
}
