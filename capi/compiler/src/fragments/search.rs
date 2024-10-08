//! # Types related to searching code
//!
//! Contains types that are returned by searches in code. Those types themselves
//! provide more convenient functionality for searching within them.

use std::ops::Deref;

use super::{Branch, BranchLocation, Function, FunctionLocation};

/// # A function that was found by a search
pub struct FoundFunction<'r> {
    /// # The function that was found
    pub function: &'r Function,

    /// # The location of the function that was found
    pub location: FunctionLocation,
}

impl Deref for FoundFunction<'_> {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        self.function
    }
}

/// # A branch that was found by a search
pub struct FoundBranch<'r> {
    /// # The branch that was found
    pub branch: &'r Branch,

    /// # The location of the branch that was found
    pub location: BranchLocation,
}

impl Deref for FoundBranch<'_> {
    type Target = Branch;

    fn deref(&self) -> &Self::Target {
        self.branch
    }
}
