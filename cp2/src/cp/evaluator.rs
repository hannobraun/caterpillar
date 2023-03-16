pub fn evaluate(
    tokens: Vec<String>,
    data_stack: &mut Vec<bool>,
) -> Result<(), Error> {
    for token in tokens {
        match token.as_str() {
            "true" => data_stack.push(true),
            "false" => data_stack.push(false),
            "not" => match data_stack.pop() {
                Some(x) => data_stack.push(!x),
                None => return Err(Error::PopFromEmptyStack),
            },
            _ => {
                return Err(Error::UnexpectedToken(token));
            }
        }
    }

    Ok(())
}

pub enum Error {
    PopFromEmptyStack,
    UnexpectedToken(String),
}
