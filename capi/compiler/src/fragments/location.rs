/// # Code fragments, tracked by their location
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FragmentsByLocation {}

pub struct FragmentLocation {
    pub function_index_in_root_context: FunctionIndexInRootContext,
    pub fragment_indices: Vec<(BranchIndex, FragmentIndexInBranchBody)>,
}

/// # The index of a named function in the root context
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct FunctionIndexInRootContext(pub u32);

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
