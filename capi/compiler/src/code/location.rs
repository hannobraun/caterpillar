use std::fmt;

use super::{Branch, Fragment, Function, Index, NamedFunctions};

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
pub struct FragmentLocation {
    pub parent: Box<BranchLocation>,
    pub index: Index<Fragment>,
}

impl FragmentLocation {
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
    location: &'r FragmentLocation,
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
            "branch {} in {}",
            self.location.index,
            self.location.parent.display(self.named_functions),
        )
    }
}

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
    AnonymousFunction { location: FragmentLocation },
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
