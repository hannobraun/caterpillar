use std::{
    collections::BTreeMap,
    iter,
    ops::{Deref, DerefMut},
};

use super::{hash::Hash, Branch, Fragment, FragmentId, Function};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    /// The root fragment that indirectly points to all other fragments
    pub root: FragmentId,

    pub map: FragmentMap,
}

impl Deref for Fragments {
    type Target = FragmentMap;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Fragments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FragmentMap {
    fragments_by_id: BTreeMap<FragmentId, Fragment>,
    ids_by_hash: BTreeMap<Hash<FragmentId>, FragmentId>,
}

impl FragmentMap {
    pub fn insert(&mut self, id: FragmentId, fragment: Fragment) {
        self.ids_by_hash.insert(id.hash(), id);
        self.fragments_by_id.insert(id, fragment.clone());
    }

    pub fn remove(&mut self, id: &FragmentId) -> Option<Fragment> {
        self.ids_by_hash.remove(&id.hash());
        self.fragments_by_id.remove(id)
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

    /// Find the named function that contains the provided fragment
    ///
    /// Any fragment that is syntactically a part of the named function will do.
    /// This specifically includes fragments within anonymous functions that are
    /// defined in the named function.
    ///
    /// Returns the found function, as well as the branch within which the
    /// fragment was found.
    pub fn find_named_function_by_fragment_in_body(
        &self,
        fragment_in_body: &FragmentId,
    ) -> Option<(FoundFunction, &Branch)> {
        let mut current_fragment = *fragment_in_body;

        loop {
            let previous = self
                .ids_by_hash
                .values()
                .find(|id| id.next == Some(current_fragment.hash()));

            if let Some(id) = previous {
                // There's a previous fragment. Continue the search there.
                current_fragment = *id;
                continue;
            }

            // If there's no previous fragment, this might be the first fragment
            // in a branch of a function.
            let function = self
                .fragments_by_id
                .iter()
                .filter_map(|(id, fragment)| match &fragment {
                    Fragment::Function { function } => Some((id, function)),
                    _ => None,
                })
                .find_map(|(id, function)| {
                    let branch = function
                        .branches
                        .iter()
                        .find(|branch| branch.start == current_fragment)?;
                    Some((*id, function, branch))
                });

            if let Some((id, function, branch)) = function {
                // We have found a function!

                if function.name.is_some() {
                    // It's a named function! Exactly what we've been looking
                    // for.
                    return Some((FoundFunction { id, function }, branch));
                } else {
                    // An anonymous function. Let's continue our search in the
                    // context where it was defined.
                    current_fragment = id;
                    continue;
                }
            }

            // We haven't found anything. Not even a new fragment to look at.
            // We're done here.
            break None;
        }
    }

    pub fn iter_from(
        &self,
        start: FragmentId,
    ) -> impl Iterator<Item = (FragmentId, &Fragment)> {
        let mut next = Some(start);

        iter::from_fn(move || {
            let id = next.take()?;
            let fragment = self.fragments_by_id.get(&id)?;

            next = id.next.and_then(|hash_of_id| {
                self.ids_by_hash.get(&hash_of_id).copied()
            });

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
