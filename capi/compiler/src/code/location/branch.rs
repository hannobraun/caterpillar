use std::{fmt, iter};

use crate::code::{
    syntax::{Branch, Expression, Function, Member, SyntaxTree},
    Index,
};

use super::{
    located::HasLocation, ExpressionLocation, FunctionLocation, Located,
};

impl HasLocation for Branch {
    type Location = BranchLocation;
}

impl<'r> Located<&'r Branch> {
    /// # Iterate over the expressions in the branch's body
    pub fn body(
        self,
    ) -> impl DoubleEndedIterator<Item = Located<&'r Expression>> {
        let location = self.location.clone();

        self.body.iter().map(move |(&index, syntax_node)| {
            let Member::Expression { expression, .. } = syntax_node;

            Located {
                fragment: expression,
                location: ExpressionLocation {
                    parent: Box::new(location.clone()),
                    index,
                },
            }
        })
    }

    /// # Iterate over all local functions in this branch, recursively
    pub fn all_local_functions(
        self,
    ) -> impl Iterator<Item = Located<&'r Function>> {
        self.body()
            .filter_map(|expression| expression.into_local_function())
            .flat_map(|function| {
                iter::once(function.clone())
                    .chain(function.all_local_functions())
            })
    }
}

/// # The location of a branch in the source code
#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct BranchLocation {
    pub parent: Box<FunctionLocation>,
    pub index: Index<Branch>,
}

impl BranchLocation {
    /// # Create a helper that implements [`fmt::Display`]
    pub fn display<'r>(
        &'r self,
        syntax_tree: &'r SyntaxTree,
    ) -> BranchLocationDisplay<'r> {
        BranchLocationDisplay {
            location: self,
            syntax_tree,
        }
    }
}

/// # Helper struct to display [`BranchLocation`]
///
/// Implements [`fmt::Display`], which [`BranchLocation`] itself doesn't.
pub struct BranchLocationDisplay<'r> {
    location: &'r BranchLocation,
    syntax_tree: &'r SyntaxTree,
}

impl fmt::Display for BranchLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "branch {} of {}",
            self.location.index,
            self.location.parent.display(self.syntax_tree),
        )
    }
}
