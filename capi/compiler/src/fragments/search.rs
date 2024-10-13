//! # Types related to searching code
//!
//! Contains types that are returned by searches in code. Those types themselves
//! provide more convenient functionality for searching within them.

use std::ops::Deref;

use super::{
    Branch, BranchLocation, Fragment, FragmentLocation, Function,
    FunctionLocation,
};

/// # The result of a search, alongside search-specific metadata
///
/// This type provides a convenient way for searches to return that additional
/// metadata, without complicating the handling of the return value too much.
///
/// In addition, it provides a target for attaching addition result-specific
/// APIs to, that would otherwise be very inconvenient to access.
pub struct Find<T, M> {
    /// # The result of the search
    pub find: T,

    /// # The additional search-specific metadata
    pub metadata: M,
}

impl<T, M> Deref for Find<T, M> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.find
    }
}

impl Find<Function, FunctionLocation> {
    /// # Iterate over the function's branches
    pub fn branches(
        &self,
    ) -> impl Iterator<Item = Find<Branch, BranchLocation>> {
        let function = &self.find;
        let location = self.metadata.clone();

        function
            .branches
            .clone()
            .into_iter()
            .map(move |(index, branch)| Find {
                find: branch,
                metadata: BranchLocation {
                    parent: Box::new(location.clone()),
                    index,
                },
            })
    }

    /// # Access the function's single branch
    ///
    /// Returns `None`, if the function does not have exactly one branch.
    pub fn find_single_branch(&self) -> Option<Find<Branch, BranchLocation>> {
        let function = &self.find;
        let location = self.metadata.clone();

        if function.branches.len() > 1 {
            return None;
        }

        function
            .branches
            .first_key_value()
            .map(|(&index, branch)| Find {
                find: branch.clone(),
                metadata: BranchLocation {
                    parent: Box::new(location),
                    index,
                },
            })
    }
}

impl Find<Branch, BranchLocation> {
    /// # Iterate over the fragments in the branch's body
    pub fn body(
        &self,
    ) -> impl Iterator<Item = Find<Fragment, FragmentLocation>> {
        let location = self.metadata.clone();
        self.body
            .clone()
            .into_iter()
            .map(move |(index, fragment)| Find {
                find: fragment,
                metadata: FragmentLocation {
                    parent: Box::new(location.clone()),
                    index,
                },
            })
    }
}

impl Find<Fragment, FragmentLocation> {
    /// # Consume the found fragment, returning its location
    pub fn into_location(self) -> FragmentLocation {
        self.metadata
    }
}
