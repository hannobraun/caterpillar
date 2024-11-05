use std::fmt;

use super::{Branch, Fragment, Function, Index};

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

impl fmt::Display for FragmentLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fragment {} in {}", self.index, self.parent)
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

impl fmt::Display for BranchLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "branch {} in {}", self.index, self.parent.display())
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
    pub fn display(&self) -> FunctionLocationDisplay {
        FunctionLocationDisplay { location: self }
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
}

impl fmt::Display for FunctionLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.location {
            FunctionLocation::NamedFunction { index } => {
                write!(f, "named function {index}")?;
            }
            FunctionLocation::AnonymousFunction { location } => {
                write!(f, "anonymous function at {location}")?;
            }
        }

        Ok(())
    }
}
