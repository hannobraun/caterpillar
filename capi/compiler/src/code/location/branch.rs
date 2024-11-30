use std::fmt;

use crate::code::{syntax::SyntaxTree, Branch, Expression, Index};

use super::{
    located::HasLocation, ExpressionLocation, FunctionLocation, Located,
};

impl HasLocation for Branch {
    type Location = BranchLocation;
}

impl Located<&Branch> {
    /// # Iterate over the expressions in the branch's body
    pub fn body(
        &self,
    ) -> impl DoubleEndedIterator<Item = Located<&Expression>> {
        let location = self.location.clone();

        self.body.iter().map(move |(&index, expression)| Located {
            fragment: expression,
            location: ExpressionLocation {
                parent: Box::new(location.clone()),
                index,
            },
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
