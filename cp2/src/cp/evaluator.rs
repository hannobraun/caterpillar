use super::data_stack::{DataStack, PopFromEmptyStack};

pub fn evaluate(
    tokens: Vec<String>,
    data_stack: &mut DataStack,
) -> Result<(), Error> {
    for token in tokens {
        match token.as_str() {
            "true" => data_stack.push(true),
            "false" => data_stack.push(false),
            "not" => match data_stack.pop() {
                Ok(x) => data_stack.push(!x),
                Err(PopFromEmptyStack) => return Err(Error::PopFromEmptyStack),
            },
            _ => {
                return Err(Error::UnexpectedToken(token));
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Tried to pop value from empty stack")]
    PopFromEmptyStack,

    #[error("Unexpected token: `{0}`")]
    UnexpectedToken(String),
}
