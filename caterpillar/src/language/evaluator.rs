use std::collections::VecDeque;

use super::{
    tokenizer::Token,
    values::{Color, Value},
};

pub fn evaluate(
    tokens: impl Iterator<Item = Token>,
    stack: &mut VecDeque<Value>,
) {
    for token in tokens {
        let Token::Fn { name: function } = token;

        match function.as_str() {
            "color" => {
                let b = stack.pop_back();
                let g = stack.pop_back();
                let r = stack.pop_back();

                if let (
                    Some(Value::U8(r)),
                    Some(Value::U8(g)),
                    Some(Value::U8(b)),
                ) = (r, g, b)
                {
                    let color = Color { r, g, b };
                    let value = Value::Color(color);

                    stack.push_back(value);
                }
            }
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
