use std::collections::BTreeSet;

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
    /// more sense to associate a name with a function where it is defined. As
    /// of this writing, this would be the root scope for all named functions.
    /// In the future, it might be any module.
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

    /// # Values captured by the function from a parent scope
    ///
    /// All functions in Caterpillar are closures that can use values from
    /// parent scopes. The names of those values are stored here.
    ///
    /// ## Implementation Note
    ///
    /// Right now, this is always empty for named functions, and only used for
    /// anonymous ones. This is just a snapshot of the current situation,
    /// however, and will most likely change as the language becomes less
    /// limited.
    ///
    /// This field refers to the captured values by name. It is likely that
    /// there are advantages to instead referring to them by fragment ID.
    pub environment: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Branch {
    pub parameters: Parameters,
    pub start: FragmentId,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Parameters {
    pub inner: Vec<Pattern>,
}
