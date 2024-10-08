#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FragmentLocation {
    pub parent: Box<BranchLocation>,
    pub index: FragmentIndexInBranchBody,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BranchLocation {
    pub parent: Box<FunctionLocation>,
    pub index: BranchIndex,
}

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum FunctionLocation {
    NamedFunction { index: FunctionIndexInRootContext },
    AnonymousFunction { location: FragmentLocation },
}

/// # The index of a named function in the root context
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct FunctionIndexInRootContext(pub u32);

/// # An index into the list of functions in a cluster
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct FunctionIndexInCluster(pub u32);

/// # The index of a branch within a function
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct BranchIndex(pub u32);

/// # The index of a fragment in a branch body
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct FragmentIndexInBranchBody(pub u32);
