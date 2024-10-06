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

/// # An index into the list of all named functions
///
/// Assumes named functions are ordered as they appear in the source code.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct NamedFunctionIndex(pub u32);
