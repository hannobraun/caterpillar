use super::{
    data_stack::{DataStack, PopFromEmptyStack},
    parser::{Expression, Expressions},
};

pub fn evaluate(
    expressions: Expressions,
    data_stack: &mut DataStack,
) -> Result<(), Error> {
    for Expression::Word(word) in expressions.0 {
        match word.as_str() {
            "drop" => data_stack.pop().map(|_| ())?,
            "true" => data_stack.push(true),
            "false" => data_stack.push(false),
            "not" => {
                let arg = data_stack.pop()?;
                let value = !arg;
                data_stack.push(value);
            }
            _ => {
                return Err(Error::UnknownWord(word));
            }
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    PopFromEmptyStack(#[from] PopFromEmptyStack),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}
