//! # Types related to searching code
//!
//! Contains types that are returned by searches in code. Those types themselves
//! provide more convenient functionality for searching within them.

use std::ops::Deref;

use super::{
    Branch, BranchLocation, Expression, ExpressionLocation, FunctionLocation,
    Index, NamedFunction,
};

/// # The result of a search, alongside search-specific metadata
///
/// This type provides a convenient way for searches to return that additional
/// metadata, without complicating the handling of the return value too much.
///
/// In addition, it provides a target for attaching addition result-specific
/// APIs to, that would otherwise be very inconvenient to access.
#[derive(Debug)]
pub struct Located<T, M> {
    /// # The result of the search
    pub find: T,

    /// # The additional search-specific metadata
    pub metadata: M,
}

impl<T, M> Deref for Located<T, M> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.find
    }
}

impl<F> Located<F, Index<NamedFunction>> {
    /// # Access the index of the found function
    ///
    /// This is a convenience accessor, to make code that would otherwise access
    /// `metadata` directly more readable.
    pub fn index(&self) -> Index<NamedFunction> {
        self.metadata
    }

    /// # Access the location of the found function
    pub fn location(&self) -> FunctionLocation {
        let index = self.metadata;
        index.into()
    }
}

impl<M> Located<&NamedFunction, M>
where
    M: Clone + Into<FunctionLocation>,
{
    /// # Iterate over the function's branches
    pub fn branches(
        &self,
    ) -> impl Iterator<Item = Located<Branch, BranchLocation>> {
        let function = &self.find;
        let location = self.metadata.clone().into();

        function.inner.branches.clone().into_iter().map(
            move |(index, branch)| Located {
                find: branch,
                metadata: BranchLocation {
                    parent: Box::new(location.clone()),
                    index,
                },
            },
        )
    }

    /// # Access the function's single branch
    ///
    /// Returns `None`, if the function does not have exactly one branch.
    pub fn find_single_branch(
        &self,
    ) -> Option<Located<Branch, BranchLocation>> {
        let function = &self.find;
        let location = self.metadata.clone().into();

        if function.inner.branches.len() > 1 {
            return None;
        }

        function
            .inner
            .branches
            .first_key_value()
            .map(|(&index, branch)| Located {
                find: branch.clone(),
                metadata: BranchLocation {
                    parent: Box::new(location),
                    index,
                },
            })
    }
}

impl Located<Branch, BranchLocation> {
    /// # Access the branch's location
    pub fn location(&self) -> &BranchLocation {
        &self.metadata
    }

    /// # Iterate over the expressions in the branch's body
    pub fn body(
        &self,
    ) -> impl Iterator<Item = Located<Expression, ExpressionLocation>> {
        let location = self.metadata.clone();
        self.body
            .clone()
            .into_iter()
            .map(move |(index, expression)| Located {
                find: expression,
                metadata: ExpressionLocation {
                    parent: Box::new(location.clone()),
                    index,
                },
            })
    }

    /// # Consume the found branch, returning its location
    pub fn into_location(self) -> BranchLocation {
        self.metadata
    }
}

impl Located<Expression, ExpressionLocation> {
    /// # Access the expression's location
    pub fn location(&self) -> &ExpressionLocation {
        &self.metadata
    }

    /// # Consume the found expression, returning its location
    pub fn into_location(self) -> ExpressionLocation {
        self.metadata
    }
}
