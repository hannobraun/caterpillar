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
