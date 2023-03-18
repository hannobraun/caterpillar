use super::{
    data_stack::{DataStack, PopFromEmptyStack},
    tokenizer::{Token, Tokens},
};

pub fn evaluate(
    tokens: Tokens,
    data_stack: &mut DataStack,
) -> Result<(), Error> {
    for Token::Word(token) in tokens.0 {
        match token.as_str() {
            "drop" => data_stack.pop().map(|_| ())?,
            "true" => data_stack.push(true),
            "false" => data_stack.push(false),
            "not" => {
                let arg = data_stack.pop()?;
                let value = !arg;
                data_stack.push(value);
            }
            _ => {
                return Err(Error::UnexpectedToken(token));
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PopFromEmptyStack(#[from] PopFromEmptyStack),

    #[error("Unexpected token: `{0}`")]
    UnexpectedToken(String),
}
