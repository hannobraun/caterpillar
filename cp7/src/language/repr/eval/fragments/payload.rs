use crate::language::repr::eval::value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FragmentPayload {
    Value(Value),
    Word(String),
    Terminator,
}

impl FragmentPayload {
    pub fn display_short(&self) -> String {
        match self {
            Self::Value(value) => {
                let value = value.display_short();
                format!("value `{value}`")
            }
            Self::Word(word) => format!("word `{word}`"),
            Self::Terminator => "terminator".to_string(),
        }
    }

    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            Self::Value(Value::Block { start }) => {
                hasher.update(b"block");
                hasher.update(start.hash.as_bytes());
            }
            Self::Value(Value::Number(number)) => {
                hasher.update(b"number");
                hasher.update(&number.to_le_bytes());
            }
            Self::Value(Value::Symbol(symbol)) => {
                hasher.update(b"symbol");
                hasher.update(symbol.as_bytes());
            }
            Self::Word(word) => {
                hasher.update(b"word");
                hasher.update(word.as_bytes());
            }
            Self::Terminator => {
                hasher.update(b"terminator");
            }
        }
    }
}
