use std::collections::BTreeMap;

use super::{
    call_stack::Functions,
    data_stack::{self, DataStack, Value},
    parser::{Expression, Expressions},
};

pub fn evaluate(
    expressions: Expressions,
    data_stack: &mut DataStack,
) -> Result<(), Error> {
    let mut functions = Functions::new();
    let mut bindings = BTreeMap::new();

    for expression in expressions.0 {
        match expression {
            Expression::Function { name, body } => {
                functions.insert(name, body);
            }
            Expression::Binding(mut names) => {
                while let Some(name) = names.pop() {
                    let value = data_stack.pop_any()?;
                    bindings.insert(name, value);
                }
            }
            Expression::Array(expressions) => {
                data_stack.mark();
                evaluate(expressions, data_stack)?;
                let values = data_stack.drain_values_from_mark().collect();
                let array = Value::Array(values);
                data_stack.push(array);
            }
            Expression::Block(expressions) => {
                data_stack.push(Value::Block(expressions));
            }
            Expression::Word(word) => match word.as_str() {
                "clone" => {
                    let original = data_stack.pop_any()?;
                    let clone = original.clone();

                    data_stack.push(original);
                    data_stack.push(clone);
                }
                "drop" => data_stack.pop_any().map(|_| ())?,
                "eval" => {
                    let block = data_stack.pop_block()?;
                    evaluate(block, data_stack)?;
                }
                "if" => {
                    let else_ = data_stack.pop_block()?;
                    let then = data_stack.pop_block()?;
                    let condition = data_stack.pop_bool()?;

                    if condition {
                        evaluate(then, data_stack)?;
                    } else {
                        evaluate(else_, data_stack)?;
                    }
                }
                "true" => data_stack.push(true),
                "false" => data_stack.push(false),
                "not" => {
                    let arg = data_stack.pop_bool()?;
                    let value = !arg;
                    data_stack.push(value);
                }
                "unwrap" => {
                    let array = data_stack.pop_array()?;
                    for value in array {
                        data_stack.push(value);
                    }
                }
                _ => {
                    if let Some(body) = functions.get(&word).cloned() {
                        evaluate(body, data_stack)?;
                        continue;
                    }
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
