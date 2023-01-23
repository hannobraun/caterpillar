use std::iter;

use super::{tokenizer::Token, values::Value};

pub enum Operation {
    /// Push a value to the stack
    Push(Value),

    /// Pop a value from the stack
    Pop,
}

pub fn evaluate(
    mut tokens: impl Iterator<Item = Token>,
) -> impl Iterator<Item = Operation> {
    iter::from_fn(move || {
        let Token::Fn { name: function } = tokens.next()?;

        match function.as_str() {
            "drop" => {
                let op = Operation::Pop;
                return Some(op);
            }
            _ => {
                if let Ok(value) = function.parse::<u8>() {
                    let value = Value::U8(value);
                    let op = Operation::Push(value);
                    return Some(op);
                }
            }
        }

        None
    })
}
