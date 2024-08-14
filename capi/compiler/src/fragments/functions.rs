use crate::syntax::Pattern;

use super::FragmentId;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub members: Vec<Branch>,
}

impl Function {
    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        // Let's destructure `self`, so we don't forget any fields.
        let Self { name, members } = self;

        hasher.update(name.as_bytes());
        for function in members {
            function.hash(hasher);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Branch {
    pub arguments: Arguments,
    pub start: FragmentId,
}

impl Branch {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        // Let's destructure `self`, so we don't forget any fields.
        let Self { arguments, start } = self;

        arguments.hash(hasher);
        start.hash(hasher);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Arguments {
    pub inner: Vec<Pattern>,
}

impl Arguments {
    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        for argument in &self.inner {
            match argument {
                Pattern::Identifier { name } => {
                    hasher.update(b"identifier pattern");
                    hasher.update(name.as_bytes());
                }
                Pattern::Literal { value } => {
                    hasher.update(b"literal pattern");
                    hasher.update(&value.0);
                }
            }
        }
    }
}
