use super::{
    call_stack::CallStack,
    data_stack::{self, DataStack, Value},
    parser::{Expression, SyntaxTree},
};

pub fn evaluate(
    syntax_tree: SyntaxTree,
    call_stack: &mut CallStack,
    data_stack: &mut DataStack,
) -> Result<(), ErrorKind> {
    let mut stack_frame = call_stack.new_stack_frame();

    for expression in &syntax_tree {
        match expression {
            Expression::Function { name, body } => {
                stack_frame.functions.insert(name.clone(), body.clone());
            }
            Expression::Binding(names) => {
                for name in names.iter().rev() {
                    let value = data_stack.pop_any()?;
                    stack_frame.bindings.insert(name.clone(), value);
                }
            }
            Expression::Array { syntax_tree } => {
                data_stack.mark();
                evaluate(syntax_tree.clone(), call_stack, data_stack)?;
                let values = data_stack.drain_values_from_mark().collect();
                let array = Value::Array(values);
                data_stack.push(array);
            }
            Expression::Block { syntax_tree } => {
                data_stack.push(Value::Block {
                    syntax_tree: syntax_tree.clone(),
                });
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
                    evaluate(block, call_stack, data_stack)?;
                }
                "if" => {
                    let else_ = data_stack.pop_block()?;
                    let then = data_stack.pop_block()?;
                    let condition = data_stack.pop_bool()?;

                    if condition {
                        evaluate(then, call_stack, data_stack)?;
                    } else {
                        evaluate(else_, call_stack, data_stack)?;
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
                    if let Some(body) = stack_frame.functions.get(word).cloned()
                    {
                        evaluate(body, call_stack, data_stack)?;
                        continue;
                    }
                    if let Some(value) = stack_frame.bindings.remove(word) {
                        data_stack.push(value);
                        continue;
                    }

                    return Err(ErrorKind::UnknownWord(word.clone()));
                }
            },
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error(transparent)]
    DataStack(#[from] data_stack::Error),

    #[error("Unknown word: `{0}`")]
    UnknownWord(String),
}
