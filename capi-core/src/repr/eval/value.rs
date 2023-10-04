use std::fmt;

use enum_variant_type::EnumVariantType;

use crate::repr::eval::fragments::FragmentId;

#[derive(Clone, Debug)]
pub struct Value {
    pub kind: ValueKind,
    pub fragment: Option<FragmentId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, EnumVariantType)]
#[evt(derive(Clone, Debug, Eq, PartialEq))]
pub enum ValueKind {
    Block { start: FragmentId },
    Number(i64),
    Symbol(String),
    Text(String),
}

impl ValueKind {
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
            ValueKind::Block { start } => {
                format!("{{ {} }}", start.display_short())
            }
            value => value.to_string(),
        }
    }
}

impl fmt::Display for ValueKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ValueKind::Block { start, .. } => write!(f, "{{ {start} }}"),
            ValueKind::Number(number) => write!(f, "{number}"),
            ValueKind::Symbol(symbol) => write!(f, ":{symbol}"),
            ValueKind::Text(text) => write!(f, "\"{text}\""),
        }
    }
}

pub trait Type: TryFrom<ValueKind, Error = ValueKind> {
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

impl Type for Text {
    const NAME: &'static str = "test";
}

#[derive(Debug, thiserror::Error)]
#[error("Expected {expected}, found `{value}`")]
pub struct TypeError {
    pub value: ValueKind,
    pub expected: &'static str,
}
