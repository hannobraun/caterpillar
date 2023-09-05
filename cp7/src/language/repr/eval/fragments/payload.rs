use crate::language::repr::eval::value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FragmentPayload {
    Value(Value),
    Word(String),
}

impl FragmentPayload {
    pub fn display_short(&self) -> String {
        match self {
            FragmentPayload::Value(value) => {
                let value = value.display_short();
                format!("value `{value}`")
            }
            FragmentPayload::Word(word) => format!("word `{word}`"),
        }
    }

    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            FragmentPayload::Value(Value::Block { start }) => {
                hasher.update(b"block");
                if let Some(start) = start {
                    hasher.update(start.hash.as_bytes());
                }
            }
            FragmentPayload::Value(Value::Number(number)) => {
                hasher.update(b"number");
                hasher.update(&number.to_le_bytes());
            }
            FragmentPayload::Value(Value::Symbol(symbol)) => {
                hasher.update(b"symbol");
                hasher.update(symbol.as_bytes());
            }
            FragmentPayload::Word(word) => {
                hasher.update(b"word");
                hasher.update(word.as_bytes());
            }
        }
    }
}
