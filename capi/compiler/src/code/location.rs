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
