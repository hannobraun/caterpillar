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
    pub fn branches(&self) -> impl Iterator<Item = FoundBranch> {
        let function = &self.find;
        let location = self.metadata.clone();

        function
            .branches
            .clone()
            .into_iter()
            .map(move |(index, branch)| FoundBranch {
                branch,
                location: BranchLocation {
                    parent: Box::new(location.clone()),
                    index,
                },
            })
    }

    /// # Access the function's single branch
    ///
    /// Returns `None`, if the function does not have exactly one branch.
    pub fn find_single_branch(&self) -> Option<FoundBranch> {
        let function = &self.find;
        let location = self.metadata.clone();

        if function.branches.len() > 1 {
            return None;
        }

        function.branches.first_key_value().map(|(&index, branch)| {
            FoundBranch {
                branch: branch.clone(),
                location: BranchLocation {
                    parent: Box::new(location),
                    index,
                },
            }
        })
    }
}

/// # A branch that was found by a search
pub struct FoundBranch {
    /// # The branch that was found
    pub branch: Branch,

    /// # The location of the branch that was found
    pub location: BranchLocation,
}

impl FoundBranch {
    /// # Iterate over the fragments in the branch's body
    pub fn body(&self) -> impl Iterator<Item = FoundFragment> {
        let location = self.location.clone();
        self.body.clone().into_iter().map(move |(index, fragment)| {
            FoundFragment {
                fragment,
                location: FragmentLocation {
                    parent: Box::new(location.clone()),
                    index,
                },
            }
        })
    }
}

impl Deref for FoundBranch {
    type Target = Branch;

    fn deref(&self) -> &Self::Target {
        &self.branch
    }
}

/// # A fragment that was found by a search
pub struct FoundFragment {
    /// # The fragment that was found
    pub fragment: Fragment,

    /// # The location of the fragment that was found
    pub location: FragmentLocation,
}

impl Deref for FoundFragment {
    type Target = Fragment;

    fn deref(&self) -> &Self::Target {
        &self.fragment
    }
}
