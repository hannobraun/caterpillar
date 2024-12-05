use std::fmt;

use crate::code::{
    syntax::{Expression, Function, Member, SyntaxTree},
    Index,
};

use super::{located::HasLocation, BranchLocation, FunctionLocation, Located};

impl HasLocation for Expression {
    type Location = MemberLocation;
}

impl<'r> Located<&'r Expression> {
    /// # Convert the located expression into a located local function
    pub fn into_local_function(self) -> Option<Located<&'r Function>> {
        self.fragment.as_local_function().map(|function| Located {
            fragment: function,
            location: FunctionLocation::AnonymousFunction {
                location: self.location.clone(),
            },
        })
    }
}

/// # The location of a member of a branch body
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
pub struct MemberLocation {
    pub parent: Box<BranchLocation>,
    pub index: Index<Member>,
}

impl MemberLocation {
    /// # Create a helper that implements [`fmt::Display`]
    pub fn display<'r>(
        &'r self,
        syntax_tree: &'r SyntaxTree,
    ) -> MemberLocationDisplay<'r> {
        MemberLocationDisplay {
            location: self,
            syntax_tree,
        }
    }
}

/// # Helper struct to display [`ExpressionLocation`]
///
/// Implements [`fmt::Display`], which [`ExpressionLocation`] itself doesn't.
pub struct MemberLocationDisplay<'r> {
    location: &'r MemberLocation,
    syntax_tree: &'r SyntaxTree,
}

impl fmt::Display for MemberLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "expression {}\n    in {}",
            self.location.index,
            self.location.parent.display(self.syntax_tree)
        )
    }
}
