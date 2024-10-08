use std::{collections::BTreeMap, iter, ops::Deref};

use crate::hash::{Hash, NextNeighbor, PrevNeighbor};

use super::{Fragment, Function};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FragmentMap {
    fragments_by_id: BTreeMap<FragmentId, Fragment>,
    previous_to_next: BTreeMap<FragmentId, FragmentId>,
    next_to_previous: BTreeMap<FragmentId, FragmentId>,
}

impl FragmentMap {
    pub fn insert(
        &mut self,
        id: FragmentId,
        fragment: Fragment,
        previous: Option<FragmentId>,
        next: Option<FragmentId>,
    ) {
        assert_eq!(
            id.content,
            Hash::new(&fragment),
            "`Fragment` must match the `FragmentId` it is inserted under.",
        );

        self.fragments_by_id.insert(id, fragment.clone());

        if let Some(previous) = previous {
            self.next_to_previous.insert(id, previous);
        }
        if let Some(next) = next {
            self.previous_to_next.insert(id, next);
        }
    }

    pub fn get(&self, id: &FragmentId) -> Option<&Fragment> {
        self.fragments_by_id.get(id)
    }

    pub fn find_function_by_name(&self, name: &str) -> Option<FoundFunction> {
        self.fragments_by_id
            .iter()
            .filter_map(|(id, fragment)| match &fragment {
                Fragment::Function { function } => Some((*id, function)),
                _ => None,
            })
            .find_map(|(id, function)| {
                if function.name.as_deref() == Some(name) {
                    Some(FoundFunction { id, function })
                } else {
                    None
                }
            })
    }

    pub fn iter_from(
        &self,
        start: Option<FragmentId>,
    ) -> impl Iterator<Item = (FragmentId, &Fragment)> {
        let mut next = start;

        iter::from_fn(move || {
            let id = next.take()?;
            let fragment = self.fragments_by_id.get(&id)?;

            next = self.previous_to_next.get(&id).copied();

            Some((id, fragment))
        })
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
