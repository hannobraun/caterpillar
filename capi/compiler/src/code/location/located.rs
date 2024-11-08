use std::ops::Deref;

use crate::code::{Branch, Expression, Index, NamedFunction};

use super::{BranchLocation, ExpressionLocation, FunctionLocation};

/// # A fragment of code, with its location attached
#[derive(Debug)]
pub struct Located<'r, T: HasLocation> {
    /// # The code fragment
    pub fragment: &'r T,

    /// # The location of the code fragment
    pub location: T::Location,
}

impl<T: HasLocation> Deref for Located<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.fragment
    }
}

impl Located<'_, NamedFunction> {
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

    /// # Iterate over the function's branches
    pub fn branches(&self) -> impl Iterator<Item = Located<Branch>> {
        let function = &self.fragment;

        function
            .inner
            .branches
            .iter()
            .map(move |(&index, branch)| Located {
                fragment: branch,
                location: BranchLocation {
                    parent: Box::new(self.location.into()),
                    index,
                },
            })
    }

    /// # Access the function's single branch
    ///
    /// Returns `None`, if the function does not have exactly one branch.
    pub fn find_single_branch(&self) -> Option<Located<Branch>> {
        let function = &self.fragment;
        let location = self.location.into();

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

impl Located<'_, Branch> {
    /// # Iterate over the expressions in the branch's body
    pub fn body(&self) -> impl Iterator<Item = Located<Expression>> {
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

impl<T> HasLocation for &T
where
    T: HasLocation,
{
    type Location = T::Location;
}

impl<T> HasLocation for &mut T
where
    T: HasLocation,
{
    type Location = T::Location;
}
