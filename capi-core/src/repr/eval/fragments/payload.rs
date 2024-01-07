use crate::repr::eval::value::ValuePayload;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FragmentPayload {
    Value(ValuePayload),
    Word(String),

    /// Terminates a context
    ///
    /// Terminators are added to the end of every context, to make sure that no
    /// block is ever the last fragment in the context. This is required to
    /// provide unique addresses for all fragments within blocks.
    ///
    /// Please refer to the documentation of [`FragmentAddress`] for more
    /// information.
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
            Self::Value(value) => {
                hasher.update(b"value");
                value.hash(hasher);
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
