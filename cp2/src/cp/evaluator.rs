use std::collections::BTreeMap;

use super::{
    data_stack::{self, DataStack, Value},
    parser::{Expression, Expressions},
};

pub fn evaluate(
    expressions: Expressions,
    data_stack: &mut DataStack,
) -> Result<(), Error> {
    let mut bindings = BTreeMap::new();

    for expression in expressions.0 {
        match expression {
            Expression::Binding(mut names) => {
                while let Some(name) = names.pop() {
                    let value = data_stack.pop_any()?;
                    bindings.insert(name, value);
                }
            }
            Expression::Block(expressions) => {
                data_stack.push(Value::Block(expressions));
            }
            Expression::Word(word) => match word.as_str() {
                "drop" => data_stack.pop_any().map(|_| ())?,
                "eval" => {
                    let block = data_stack.pop_block()?;
                    evaluate(block, data_stack)?;
                }
                "true" => data_stack.push(true),
                "false" => data_stack.push(false),
                "not" => {
                    let arg = data_stack.pop_bool()?;
                    let value = !arg;
                    data_stack.push(value);
                }
                _ => {
                    if let Some(value) = bindings.remove(&word) {
                        data_stack.push(value);
                        continue;
                    }

                    return Err(Error::UnknownWord(word));
                }
            },
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    DataStack(#[from] data_stack::Error),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}
