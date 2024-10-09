//! # Types related to searching code
//!
//! Contains types that are returned by searches in code. Those types themselves
//! provide more convenient functionality for searching within them.

use std::ops::Deref;

use super::{
    Branch, BranchLocation, Fragment, FragmentLocation, Function,
    FunctionLocation,
};

/// # A function that was found by a search
pub struct FoundFunction {
    /// # The function that was found
    pub function: Function,

    /// # The location of the function that was found
    pub location: FunctionLocation,
}

impl FoundFunction {
    /// # Iterate over the function's branches
    pub fn branches(&self) -> impl Iterator<Item = FoundBranch> {
        let location = self.location.clone();
        self.branches
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
        if self.branches.len() > 1 {
            return None;
        }

        self.branches
            .first_key_value()
            .map(|(&index, branch)| FoundBranch {
                branch: branch.clone(),
                location: BranchLocation {
                    parent: Box::new(self.location.clone()),
                    index,
                },
            })
    }
}

impl Deref for FoundFunction {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        &self.function
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
    pub fn fragments(&self) -> impl Iterator<Item = FoundFragment> {
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
