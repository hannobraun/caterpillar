use std::iter;

use super::tokenizer::Token;

pub enum Operation {
    /// Push a value to the stack
    Push(u8),
}

pub fn evaluate(
    mut tokens: impl Iterator<Item = Token>,
) -> impl Iterator<Item = Operation> {
    iter::from_fn(move || {
        let Token::Fn { name: function } = tokens.next()?;
        let Ok(value) = function.parse::<u8>() else {
            return None;
        };
        Some(Operation::Push(value))
    })
}
