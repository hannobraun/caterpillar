use std::collections::VecDeque;

use super::{
    parser::Syntax,
    values::{Color, Value},
};

pub type Stack = VecDeque<Value>;

pub fn evaluate(tokens: impl Iterator<Item = Syntax>, stack: &mut Stack) {
    for token in tokens {
        let Syntax::Fn { name } = token;

        match name.as_str() {
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
                if let Ok(value) = name.parse::<u8>() {
                    let value = Value::U8(value);
                    stack.push_back(value);
                }
            }
        }
    }
}
