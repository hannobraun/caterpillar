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
        write!(f, "branch {} in {}", self.index, self.parent)
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

impl From<Index<Function>> for FunctionLocation {
    fn from(index: Index<Function>) -> Self {
        Self::NamedFunction { index }
    }
}

impl fmt::Display for FunctionLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FunctionLocation::NamedFunction { index } => {
                write!(f, "named function {index}")?;
            }
            FunctionLocation::AnonymousFunction { location } => {
                write!(f, "anonymous function at {location:?}")?;
            }
        }

        Ok(())
    }
}
