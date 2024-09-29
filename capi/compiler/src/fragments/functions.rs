use std::collections::BTreeSet;

use crate::syntax::Pattern;

use super::{Fragment, FragmentId, FragmentMap};

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct Function {
    /// # The name of this function, if available
    ///
    /// A name is not available for anonymous functions.
    ///
    /// ## Implementation Note
    ///
    /// This happens to work for now, but it is a stopgap. It makes more sense
    /// to associate a name with a function where it is defined. As of this
    /// writing, this would be the root scope for all named functions. In the
    /// future, it could be any module.
    ///
    /// This would also allow supporting function aliases. Right now, these
    /// would break the assumption that is encoded here, that all functions have
    /// at most one name.
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
    pub environment: BTreeSet<String>,
}

impl Function {
    /// # Expect the function to have one branch and access that
    ///
    /// This is a convenience method, designed for tests and such. It should not
    /// be used in code that requires proper error handling.
    ///
    /// ## Panics
    ///
    /// Panics, if the function does not have exactly one branch.
    pub fn expect_one_branch(&self) -> &Branch {
        assert_eq!(
            self.branches.len(),
            1,
            "Expected function to have exactly one branch."
        );

        &self.branches[0]
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct Branch {
    pub parameters: Parameters,

    /// # The first fragment of the branch's body
    ///
    /// Can be `None`, if the branch body is empty.
    pub start: Option<FragmentId>,
}

impl Branch {
    /// # Iterate over the body of this branch
    pub fn body<'r>(
        &self,
        fragments: &'r FragmentMap,
    ) -> impl Iterator<Item = (FragmentId, &'r Fragment)> {
        fragments.iter_from(self.start)
    }
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct Parameters {
    pub inner: Vec<Pattern>,
}
