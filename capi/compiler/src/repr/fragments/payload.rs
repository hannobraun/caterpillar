use std::fmt;

use capi_process::Value;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FragmentPayload {
    Binding { names: Vec<String> },
    Comment { text: String },
    Value(Value),
    Word { name: String },
}

impl FragmentPayload {
    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        match self {
            FragmentPayload::Binding { names } => {
                hasher.update(b"binding");

                for name in names {
                    hasher.update(name.as_bytes());
                }
            }
            FragmentPayload::Comment { text } => {
                hasher.update(b"comment");
                hasher.update(text.as_bytes());
            }
            FragmentPayload::Value(value) => {
                hasher.update(b"value");
                hasher.update(&value.0.to_le_bytes());
            }
            FragmentPayload::Word { name } => {
                hasher.update(b"word");
                hasher.update(name.as_bytes());
            }
        }
    }
}

impl fmt::Display for FragmentPayload {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Binding { names } => {
                write!(f, "=>")?;
                for name in names {
                    write!(f, " {name}")?;
                }
                writeln!(f, " .")
            }
            Self::Comment { text } => writeln!(f, "# {text}"),
            Self::Value(value) => write!(f, "{value}"),
            Self::Word { name } => write!(f, "{name}"),
        }
    }
}
