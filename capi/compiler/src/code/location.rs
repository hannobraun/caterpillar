use std::fmt;

use super::{Branch, Expression, Function, Index, NamedFunctions};

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
        named_functions: &'r NamedFunctions,
    ) -> FragmentLocationDisplay<'r> {
        FragmentLocationDisplay {
            location: self,
            named_functions,
        }
    }
}

/// # Helper struct to display [`FragmentLocation`]
///
/// Implements [`fmt::Display`], which [`FragmentLocation`] itself doesn't.
pub struct FragmentLocationDisplay<'r> {
    location: &'r ExpressionLocation,
    named_functions: &'r NamedFunctions,
}

impl fmt::Display for FragmentLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "fragment {}\n    in {}",
            self.location.index,
            self.location.parent.display(self.named_functions)
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
        named_functions: &'r NamedFunctions,
    ) -> BranchLocationDisplay<'r> {
        BranchLocationDisplay {
            location: self,
            named_functions,
        }
    }
}

/// # Helper struct to display [`BranchLocation`]
///
/// Implements [`fmt::Display`], which [`BranchLocation`] itself doesn't.
pub struct BranchLocationDisplay<'r> {
    location: &'r BranchLocation,
    named_functions: &'r NamedFunctions,
}

impl fmt::Display for BranchLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "branch {} of {}",
            self.location.index,
            self.location.parent.display(self.named_functions),
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
    NamedFunction { index: Index<Function> },
    AnonymousFunction { location: ExpressionLocation },
}

impl FunctionLocation {
    /// # Create a helper that implements [`fmt::Display`]
    pub fn display<'r>(
        &'r self,
        named_functions: &'r NamedFunctions,
    ) -> FunctionLocationDisplay<'r> {
        FunctionLocationDisplay {
            location: self,
            named_functions,
        }
    }
}

impl From<Index<Function>> for FunctionLocation {
    fn from(index: Index<Function>) -> Self {
        Self::NamedFunction { index }
    }
}

/// # Helper struct to display [`FunctionLocation`]
///
/// Implements [`fmt::Display`], which [`FunctionLocation`] itself doesn't.
pub struct FunctionLocationDisplay<'r> {
    location: &'r FunctionLocation,
    named_functions: &'r NamedFunctions,
}

impl fmt::Display for FunctionLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.location {
            FunctionLocation::NamedFunction { index } => {
                let name = self
                    .named_functions
                    .find_by_index(index)
                    .expect("Named function referred to be index must exist")
                    .name
                    .as_ref()
                    .expect("Named function must have a name");

                write!(f, "named function `{name}`")?;
            }
            FunctionLocation::AnonymousFunction { location } => {
                write!(
                    f,
                    "anonymous function at {}",
                    location.display(self.named_functions),
                )?;
            }
        }

        Ok(())
    }
}
