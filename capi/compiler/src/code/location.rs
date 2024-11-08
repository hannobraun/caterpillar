use std::fmt;

use super::{Branch, Expression, Functions, Index, NamedFunction};

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
        functions: &'r Functions,
    ) -> BranchLocationDisplay<'r> {
        BranchLocationDisplay {
            location: self,
            functions,
        }
    }
}

/// # Helper struct to display [`BranchLocation`]
///
/// Implements [`fmt::Display`], which [`BranchLocation`] itself doesn't.
pub struct BranchLocationDisplay<'r> {
    location: &'r BranchLocation,
    functions: &'r Functions,
}

impl fmt::Display for BranchLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "branch {} of {}",
            self.location.index,
            self.location.parent.display(self.functions),
        )
    }
}

/// # The location of a function in the source code
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
pub enum FunctionLocation {
    NamedFunction { index: Index<NamedFunction> },
    AnonymousFunction { location: ExpressionLocation },
}

impl FunctionLocation {
    /// # Create a helper that implements [`fmt::Display`]
    pub fn display<'r>(
        &'r self,
        functions: &'r Functions,
    ) -> FunctionLocationDisplay<'r> {
        FunctionLocationDisplay {
            location: self,
            functions,
        }
    }
}

impl From<Index<NamedFunction>> for FunctionLocation {
    fn from(index: Index<NamedFunction>) -> Self {
        Self::NamedFunction { index }
    }
}

/// # Helper struct to display [`FunctionLocation`]
///
/// Implements [`fmt::Display`], which [`FunctionLocation`] itself doesn't.
pub struct FunctionLocationDisplay<'r> {
    location: &'r FunctionLocation,
    functions: &'r Functions,
}

impl fmt::Display for FunctionLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.location {
            FunctionLocation::NamedFunction { index } => {
                let name = &self
                    .functions
                    .find_named_by_index(index)
                    .expect("Named function referred to be index must exist")
                    .name;

                write!(f, "named function `{name}`")?;
            }
            FunctionLocation::AnonymousFunction { location } => {
                write!(
                    f,
                    "anonymous function at {}",
                    location.display(self.functions),
                )?;
            }
        }

        Ok(())
    }
}
