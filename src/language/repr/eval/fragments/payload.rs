use crate::language::repr::eval::value::ValueKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FragmentPayload {
    Value(ValueKind),
    Word(String),

    /// Terminates a context
    ///
    /// By convention, fragments within a block use the fragment *after* the
    /// block as their parents. This is done for practical reasons, as the ID
    /// of the next fragment is available when a parent ID is needed, as opposed
    /// to the ID of the block itself or the fragment before it. And they can't
    /// be made available either, as they depend on the block contents and that
    /// would be a circular dependency.
    ///
    /// However, that means that blocks *must not* be the last fragment in a
    /// context, or the parent will be `None`, and the items within such blocks
    /// are no longer uniquely addressable.
    ///
    /// This is why terminators exist. They terminate every context, and thus
    /// provide a unique parent for the fragments in any block.
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
            Self::Value(ValueKind::Block { start }) => {
                hasher.update(b"block");
                hasher.update(start.hash.as_bytes());
            }
            Self::Value(ValueKind::Number(number)) => {
                hasher.update(b"number");
                hasher.update(&number.to_le_bytes());
            }
            Self::Value(ValueKind::Symbol(symbol)) => {
                hasher.update(b"symbol");
                hasher.update(symbol.as_bytes());
            }
            Self::Value(ValueKind::Text(text)) => {
                hasher.update(b"text");
                hasher.update(text.as_bytes());
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
