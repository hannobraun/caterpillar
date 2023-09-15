use std::fmt;

use enum_variant_type::EnumVariantType;

use crate::language::repr::eval::fragments::FragmentId;

#[derive(Debug)]
pub struct Value {
    pub kind: ValueKind,
}

#[derive(Clone, Debug, Eq, PartialEq, EnumVariantType)]
#[evt(derive(Debug, Eq, PartialEq))]
pub enum ValueKind {
    Block { start: FragmentId },
    Number(i64),
    Symbol(String),
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

#[derive(Debug, thiserror::Error)]
#[error("Expected {expected}, found `{value}`")]
pub struct TypeError {
    pub value: ValueKind,
    pub expected: &'static str,
}
