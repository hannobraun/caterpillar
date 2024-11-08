use std::ops::Deref;

use crate::code::{Branch, Expression, Index, NamedFunction};

use super::{BranchLocation, ExpressionLocation, FunctionLocation};

/// # The result of a search, alongside search-specific metadata
///
/// This type provides a convenient way for searches to return that additional
/// metadata, without complicating the handling of the return value too much.
///
/// In addition, it provides a target for attaching addition result-specific
/// APIs to, that would otherwise be very inconvenient to access.
#[derive(Debug)]
pub struct Located<'r, T: HasLocation, M = <T as HasLocation>::Location> {
    /// # The result of the search
    pub fragment: &'r T,

    /// # The additional search-specific metadata
    pub location: M,
}

impl<T: HasLocation, M> Deref for Located<'_, T, M> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.fragment
    }
}

impl Located<'_, NamedFunction, Index<NamedFunction>> {
    /// # Access the index of the found function
    ///
    /// This is a convenience accessor, to make code that would otherwise access
    /// `metadata` directly more readable.
    pub fn index(&self) -> Index<NamedFunction> {
        self.location
    }

    /// # Access the location of the found function
    pub fn location(&self) -> FunctionLocation {
        let index = self.location;
        index.into()
    }
}

impl<M> Located<'_, NamedFunction, M>
where
    M: Clone + Into<FunctionLocation>,
{
    /// # Iterate over the function's branches
    pub fn branches(
        &self,
    ) -> impl Iterator<Item = Located<Branch, BranchLocation>> {
        let function = &self.fragment;
        let location = self.location.clone().into();

        function
            .inner
            .branches
            .iter()
            .map(move |(&index, branch)| Located {
                fragment: branch,
                location: BranchLocation {
                    parent: Box::new(location.clone()),
                    index,
                },
            })
    }

    /// # Access the function's single branch
    ///
    /// Returns `None`, if the function does not have exactly one branch.
    pub fn find_single_branch(
        &self,
    ) -> Option<Located<Branch, BranchLocation>> {
        let function = &self.fragment;
        let location = self.location.clone().into();

        if function.inner.branches.len() > 1 {
            return None;
        }

        function
            .inner
            .branches
            .first_key_value()
            .map(|(&index, branch)| Located {
                fragment: branch,
                location: BranchLocation {
                    parent: Box::new(location),
                    index,
                },
            })
    }
}

impl Located<'_, Branch, BranchLocation> {
    /// # Iterate over the expressions in the branch's body
    pub fn body(
        &self,
    ) -> impl Iterator<Item = Located<Expression, ExpressionLocation>> {
        let location = self.location.clone();
        self.body.iter().map(move |(&index, expression)| Located {
            fragment: expression,
            location: ExpressionLocation {
                parent: Box::new(location.clone()),
                index,
            },
        })
    }
}

/// # Implemented by all code fragments, to abstract over their location
pub trait HasLocation {
    /// # The location of this fragment
    type Location;
}
