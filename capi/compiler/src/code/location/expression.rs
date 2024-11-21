use std::fmt;

use crate::code::{Expression, Functions, Index};

use super::{located::HasLocation, BranchLocation, FunctionLocation, Located};

impl HasLocation for Expression {
    type Location = ExpressionLocation;
}

impl Located<&Expression> {
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
        functions: &'r Functions,
    ) -> ExpressionLocationDisplay<'r> {
        ExpressionLocationDisplay {
            location: self,
            functions,
        }
    }
}

/// # Helper struct to display [`ExpressionLocation`]
///
/// Implements [`fmt::Display`], which [`ExpressionLocation`] itself doesn't.
pub struct ExpressionLocationDisplay<'r> {
    location: &'r ExpressionLocation,
    functions: &'r Functions,
}

impl fmt::Display for ExpressionLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "expression {}\n    in {}",
            self.location.index,
            self.location.parent.display(self.functions)
        )
    }
}
