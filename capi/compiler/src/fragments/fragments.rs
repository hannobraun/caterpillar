use std::{
    collections::BTreeMap,
    iter,
    ops::{Deref, DerefMut},
};

use super::{Branch, Fragment, FragmentId, FragmentKind, Function, Payload};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    /// The root fragment that indirectly points to all other fragments
    pub root: FragmentId,

    pub inner: FragmentMap,
}

impl Deref for Fragments {
    type Target = FragmentMap;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for Fragments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct FragmentMap {
    inner: BTreeMap<FragmentId, Fragment>,
}

impl FragmentMap {
    pub fn insert(&mut self, id: FragmentId, fragment: Fragment) {
        self.inner.insert(id, fragment);
    }

    pub fn remove(&mut self, id: &FragmentId) -> Option<Fragment> {
        self.inner.remove(id)
    }

    pub fn get(&self, id: &FragmentId) -> Option<&Fragment> {
        self.inner.get(id)
    }

    pub fn find_function_by_name(&self, name: &str) -> Option<FoundFunction> {
        self.inner
            .values()
            .filter_map(|fragment| match &fragment.kind {
                FragmentKind::Payload {
                    payload: Payload::Function { function },
                    ..
                } => Some((fragment.id(), function)),
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
        fragment_id: &FragmentId,
    ) -> Option<(FoundFunction, &Branch)> {
        let mut fragment_id = *fragment_id;

        loop {
            let previous = self
                .inner
                .values()
                .find(|fragment| fragment.next == Some(fragment_id));

            if let Some(previous) = previous {
                // There's a previous fragment. Continue the search there.
                fragment_id = previous.id();
                continue;
            }

            // If there's no previous fragment, this might be the first fragment
            // in a branch of a function.
            let function = self
                .inner
                .values()
                .filter_map(|fragment| match &fragment.kind {
                    FragmentKind::Payload {
                        payload: Payload::Function { function },
                        ..
                    } => Some((fragment.id(), function)),
                    _ => None,
                })
                .find_map(|(id, function)| {
                    let branch = function
                        .branches
                        .iter()
                        .find(|branch| branch.start == fragment_id)?;
                    Some((id, function, branch))
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
                    fragment_id = id;
                    continue;
                }
            }

            // We haven't found anything. Not even a new fragment to look at.
            // We're done here.
            break None;
        }
    }

    pub fn iter_from(&self, id: FragmentId) -> impl Iterator<Item = &Fragment> {
        let mut next = Some(id);

        iter::from_fn(move || {
            let id = next.take()?;
            let fragment = self.inner.get(&id)?;

            next = fragment.next;

            Some(fragment)
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
