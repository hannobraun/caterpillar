use std::{collections::BTreeMap, iter, marker::PhantomData, ops::Deref};

use super::{Branch, Fragment, Function};

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FragmentMap {
    fragments_by_id: BTreeMap<FragmentId, Fragment>,
    ids_by_hash: BTreeMap<Hash<FragmentId>, FragmentId>,
}

impl FragmentMap {
    pub fn insert(&mut self, id: FragmentId, fragment: Fragment) {
        self.ids_by_hash.insert(Hash::new(&id), id);
        self.fragments_by_id.insert(id, fragment.clone());
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
                .find(|id| id.next == Some(Hash::new(&current_fragment)));

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
    next: Option<Hash<FragmentId>>,
    this: Hash<Fragment>,
}

impl FragmentId {
    pub fn new(
        _: Option<&FragmentId>,
        next: Option<&FragmentId>,
        this: &Fragment,
    ) -> Self {
        Self {
            next: next.map(Hash::new),
            this: Hash::new(this),
        }
    }
}

#[derive(
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
struct Hash<T> {
    hash: [u8; 32],
    _t: PhantomData<T>,
}

impl<T> Hash<T> {
    fn new(value: &T) -> Self
    where
        T: udigest::Digestable,
    {
        let hash = udigest::hash::<blake3::Hasher>(value).into();
        Self {
            hash,
            _t: PhantomData,
        }
    }
}

impl<T> Clone for Hash<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Hash<T> {}
