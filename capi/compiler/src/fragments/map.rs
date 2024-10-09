use std::{collections::BTreeMap, ops::Deref};

use crate::hash::{Hash, NextNeighbor, PrevNeighbor};

use super::{Fragment, Function};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FragmentMap {
    fragments_by_id: BTreeMap<FragmentId, Fragment>,
}

impl FragmentMap {
    pub fn insert(
        &mut self,
        id: FragmentId,
        fragment: Fragment,
        _: Option<FragmentId>,
        _: Option<FragmentId>,
    ) {
        assert_eq!(
            id.content,
            Hash::new(&fragment),
            "`Fragment` must match the `FragmentId` it is inserted under.",
        );

        self.fragments_by_id.insert(id, fragment.clone());
    }

    pub fn get(&self, id: &FragmentId) -> Option<&Fragment> {
        self.fragments_by_id.get(id)
    }
}

/// # Return type of several methods that search for functions
///
/// This type bundles the found function and its ID. It [`Deref`]s to
/// `Function`.
#[derive(Debug)]
pub struct FoundFunction<'r> {
    pub id: FragmentId,
    pub function: &'r Function,
}

impl Deref for FoundFunction<'_> {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        self.function
    }
}

/// # A unique identifier for a fragment
///
/// A fragment is identified by its contents, but also by its position within
/// the code.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct FragmentId {
    pub prev: Option<Hash<PrevNeighbor>>,

    /// # The hash of the next fragment
    ///
    /// This refers to the fragment that will be executed after the one that
    /// this `FragmentId` identifies.
    pub next: Option<Hash<NextNeighbor>>,

    /// # The hash of this fragment's content
    pub content: Hash<Fragment>,
}
