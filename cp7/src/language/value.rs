use std::fmt;

use enum_variant_type::EnumVariantType;

use super::syntax::SyntaxHandle;

#[derive(Clone, Debug, Eq, PartialEq, EnumVariantType)]
#[evt(derive(Debug, Eq, PartialEq))]
pub enum Value {
    Block(Option<SyntaxHandle>),
    Number(i64),
    Symbol(String),
}

impl Value {
    pub fn expect<T: Type>(
        self,
        expected: &'static str,
    ) -> Result<T, TypeError> {
        self.try_into()
            .map_err(|value| TypeError { value, expected })
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Block(start) => {
                write!(f, "{{")?;
                if let Some(start) = start {
                    write!(f, " {start}")?;
                }
                write!(f, " }}")?;
                Ok(())
            }
            Value::Number(number) => write!(f, "{number}"),
            Value::Symbol(symbol) => write!(f, ":{symbol}"),
        }
    }
}

pub trait Type: TryFrom<Value, Error = Value> {
    const NAME: &'static str;
}

impl Type for Block {
    const NAME: &'static str = "block";
}

impl Type for Number {
    const NAME: &'static str = "number";
}

impl Type for Symbol {
    const NAME: &'static str = "symbol";
}

#[derive(Debug, thiserror::Error)]
#[error("Expected {expected}, found `{value}`")]
pub struct TypeError {
    pub value: Value,
    pub expected: &'static str,
}
