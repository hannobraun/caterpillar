use std::fmt;

use enum_variant_type::EnumVariantType;

use crate::repr::eval::fragments::FragmentId;

#[derive(Clone, Debug)]
pub struct Value {
    pub payload: ValuePayload,
    pub fragment: Option<FragmentId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, EnumVariantType)]
#[evt(derive(Clone, Debug, Eq, PartialEq))]
pub enum ValuePayload {
    Block { start: FragmentId },
    Bool(bool),
    Number(i64),
    Symbol(String),
    Text(String),
}

impl ValuePayload {
    pub fn expect<T: Type>(
        self,
        expected: &'static str,
    ) -> Result<T, Box<TypeError>> {
        self.try_into()
            .map_err(|value| TypeError { value, expected })
            .map_err(Box::new)
    }

    pub fn display_short(&self) -> String {
        match self {
            ValuePayload::Block { start } => {
                format!("{{ {} }}", start.display_short())
            }
            value => value.to_string(),
        }
    }
}

impl fmt::Display for ValuePayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValuePayload::Block { start, .. } => write!(f, "{{ {start} }}"),
            ValuePayload::Bool(value) => write!(f, "{value}"),
            ValuePayload::Number(number) => write!(f, "{number}"),
            ValuePayload::Symbol(symbol) => write!(f, ":{symbol}"),
            ValuePayload::Text(text) => write!(f, "\"{text}\""),
        }
    }
}

pub trait Type: TryFrom<ValuePayload, Error = ValuePayload> {
    const NAME: &'static str;
}

impl Type for Block {
    const NAME: &'static str = "block";
}

impl Type for Bool {
    const NAME: &'static str = "bool";
}

impl Type for Number {
    const NAME: &'static str = "number";
}

impl Type for Symbol {
    const NAME: &'static str = "symbol";
}

impl Type for Text {
    const NAME: &'static str = "test";
}

#[derive(Debug, thiserror::Error)]
#[error("Expected {expected}, found `{value}`")]
pub struct TypeError {
    pub value: ValuePayload,
    pub expected: &'static str,
}
