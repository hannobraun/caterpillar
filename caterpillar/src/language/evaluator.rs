use std::iter;

use super::{tokenizer::Token, values::Value};

pub enum Operation {
    /// Push a value to the stack
    Push(Value),
}

pub fn evaluate(
    mut tokens: impl Iterator<Item = Token>,
) -> impl Iterator<Item = Operation> {
    iter::from_fn(move || {
        let Token::Fn { name: function } = tokens.next()?;
        let Ok(value) = function.parse::<u8>() else {
            return None;
        };
        let value = Value::U8(value);
        Some(Operation::Push(value))
    })
}
