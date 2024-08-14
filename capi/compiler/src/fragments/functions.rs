use crate::syntax::Pattern;

use super::FragmentId;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: Option<String>,

    /// # The branches of this function
    ///
    /// A function is made up of one or more branches. When a function is
    /// called, its arguments are matched against the parameters of each branch,
    /// until one branch matches. This branch is then evaluated.
    pub branches: Vec<Branch>,
}

impl Function {
    pub(super) fn hash(&self, hasher: &mut blake3::Hasher) {
        // Let's destructure `self`, so we don't forget any fields.
        let Self { name, branches } = self;

        if let Some(name) = name {
            hasher.update(name.as_bytes());
        }
        for branch in branches {
            branch.hash(hasher);
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Branch {
    pub parameters: Parameters,
    pub start: FragmentId,
}

impl Branch {
    fn hash(&self, hasher: &mut blake3::Hasher) {
        // Let's destructure `self`, so we don't forget any fields.
        let Self { parameters, start } = self;

        parameters.hash(hasher);
        start.hash(hasher);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Parameters {
    pub inner: Vec<Pattern>,
}

impl Parameters {
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
