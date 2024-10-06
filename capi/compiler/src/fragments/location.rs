use std::collections::BTreeMap;

use super::FragmentId;

/// # Code fragments, tracked by their location
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FragmentsByLocation {
    previous_to_next: BTreeMap<FragmentId, FragmentId>,
    next_to_previous: BTreeMap<FragmentId, FragmentId>,
}

impl FragmentsByLocation {
    /// # Insert a fragment
    pub fn insert(
        &mut self,
        id: FragmentId,
        previous: Option<FragmentId>,
        next: Option<FragmentId>,
    ) {
        if let Some(previous) = previous {
            self.next_to_previous.insert(id, previous);
        }
        if let Some(next) = next {
            self.previous_to_next.insert(id, next);
        }
    }
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
pub struct FragmentIndexInBranchBody(pub u32);
