use std::fmt;

use enum_variant_type::EnumVariantType;

use crate::pipeline::c_parser::SyntaxTree;

#[derive(Debug, EnumVariantType)]
#[evt(module = "value", derive(Debug))]
pub enum Value {
    Block(SyntaxTree),
    Number(i64),
    Symbol(String),
}

impl Value {
    pub fn expect<T>(self, expected: &'static str) -> Result<T, TypeError>
    where
        T: TryFrom<Value, Error = Value>,
    {
        self.try_into()
            .map_err(|value| TypeError { value, expected })
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Block(block) => write!(f, "{{ {block:?} }}"),
            Value::Number(number) => write!(f, "{number}"),
            Value::Symbol(symbol) => write!(f, ":{symbol}"),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Expected {expected}, found `{value}`")]
pub struct TypeError {
    pub value: Value,
    pub expected: &'static str,
}
