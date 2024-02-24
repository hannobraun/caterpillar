use std::fmt;

use enum_variant_type::EnumVariantType;

use crate::repr::eval::fragments::FragmentId;

#[derive(Clone, Debug)]
pub struct Value {
    pub payload: ValuePayload,
    pub fragment: Option<FragmentId>,
}

#[derive(
    Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, EnumVariantType,
)]
#[evt(derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd))]
pub enum ValuePayload {
    Array(Vec<ValuePayload>),
    Block { start: FragmentId },
    Bool(bool),
    Number(i64),
    Symbol(String),
    Text(String),
}

impl ValuePayload {
    pub fn expect<T: Type>(self) -> Result<T, TypeError> {
        self.try_into().map_err(|value| TypeError {
            value,
            expected: T::NAME,
        })
    }

    pub fn display_short(&self) -> String {
        match self {
            ValuePayload::Block { start } => {
                format!("{{ {} }}", start.display_short())
            }
            value => value.to_string(),
        }
    }

    pub(crate) fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            Self::Array(values) => {
                hasher.update(b"array");
                for value in values {
                    value.hash(hasher);
                }
            }
            Self::Block { start } => {
                hasher.update(b"block");
                hasher.update(start.hash.as_bytes());
            }
            Self::Bool(value) => {
                hasher.update(b"bool");
                hasher.update(&[(*value).into()]);
            }
            Self::Number(number) => {
                hasher.update(b"number");
                hasher.update(&number.to_le_bytes());
            }
            Self::Symbol(symbol) => {
                hasher.update(b"symbol");
                hasher.update(symbol.as_bytes());
            }
            Self::Text(text) => {
                hasher.update(b"text");
                hasher.update(text.as_bytes());
            }
        }
    }
}

impl fmt::Display for ValuePayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Array(values) => {
                write!(f, "[")?;

                for value in values {
                    write!(f, " {value}")?;
                }

                write!(f, " ]")?;

                Ok(())
            }
            Self::Block { start, .. } => write!(f, "{{ {start} }}"),
            Self::Bool(value) => write!(f, "{value}"),
            Self::Number(number) => write!(f, "{number}"),
            Self::Symbol(symbol) => write!(f, ":{symbol}"),
            Self::Text(text) => write!(f, "\"{text}\""),
        }
    }
}

pub trait Type: TryFrom<ValuePayload, Error = ValuePayload> {
    const NAME: &'static str;
}

impl Type for Array {
    const NAME: &'static str = "array";
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
    const NAME: &'static str = "text";
}

#[derive(Debug, thiserror::Error)]
#[error("Type error: expected {expected}, found `{value}`")]
pub struct TypeError {
    pub value: ValuePayload,
    pub expected: &'static str,
}
