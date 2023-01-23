use std::collections::VecDeque;

use super::{tokenizer::Token, values::Value};

pub fn evaluate(
    tokens: impl Iterator<Item = Token>,
    stack: &mut VecDeque<Value>,
) {
    for token in tokens {
        let Token::Fn { name: function } = token;

        match function.as_str() {
            "drop" => {
                stack.pop_back();
            }
            _ => {
                if let Ok(value) = function.parse::<u8>() {
                    let value = Value::U8(value);
                    stack.push_back(value);
                }
            }
        }
    }
}
