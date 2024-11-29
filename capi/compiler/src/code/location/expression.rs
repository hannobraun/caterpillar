use std::fmt;

use crate::code::{syntax::SyntaxTree, Expression, Function, Index};

use super::{located::HasLocation, BranchLocation, FunctionLocation, Located};

impl HasLocation for Expression {
    type Location = ExpressionLocation;
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

    /// # Convert the located expression to a function location, if possible
    ///
    /// Returns `None`, if the expression is not a function literal.
    pub fn to_function_location(&self) -> Option<FunctionLocation> {
        if let Expression::LocalFunction { .. } = self.fragment {
            Some(FunctionLocation::AnonymousFunction {
                location: self.location.clone(),
            })
        } else {
            None
        }
    }
}

/// # The location of an expression in the source code
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
pub struct ExpressionLocation {
    pub parent: Box<BranchLocation>,
    pub index: Index<Expression>,
}

impl ExpressionLocation {
    /// # Create a helper that implements [`fmt::Display`]
    pub fn display<'r>(
        &'r self,
        syntax_tree: &'r SyntaxTree,
    ) -> ExpressionLocationDisplay<'r> {
        ExpressionLocationDisplay {
            location: self,
            syntax_tree,
        }
    }
}

/// # Helper struct to display [`ExpressionLocation`]
///
/// Implements [`fmt::Display`], which [`ExpressionLocation`] itself doesn't.
pub struct ExpressionLocationDisplay<'r> {
    location: &'r ExpressionLocation,
    syntax_tree: &'r SyntaxTree,
}

impl fmt::Display for ExpressionLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "expression {}\n    in {}",
            self.location.index,
            self.location.parent.display(self.syntax_tree)
        )
    }
}
