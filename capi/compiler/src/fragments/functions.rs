use crate::syntax::Pattern;

use super::FragmentId;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Function {
    /// # The name of this function, if available
    ///
    /// A name is not available for anonymous functions.
    ///
    /// ## Implementation Note
    ///
    /// This happens to work for now, but it's most likely a stopgap. It makes
    /// more sense to associate a name with a function were it is defined. As of
    /// this writing, this would be the root scope for all named functions. In
    /// the future, it might be any module.
    ///
    /// This would also allow supporting function aliases, which would break the
    /// assumption that all functions have at most one name.
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
