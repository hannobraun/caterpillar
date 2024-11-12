use std::{
    collections::BTreeSet,
    ops::{Deref, DerefMut},
};

use crate::code::{
    Branch, Expression, Function, Index, NamedFunction, Pattern,
};

use super::{BranchLocation, ExpressionLocation, FunctionLocation};

/// # A fragment of code, with its location attached
#[derive(Clone, Debug)]
pub struct Located<T: HasLocation> {
    /// # The code fragment
    pub fragment: T,

    /// # The location of the code fragment
    pub location: T::Location,
}

impl<T: HasLocation> Deref for Located<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.fragment
    }
}

impl<T: HasLocation> DerefMut for Located<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.fragment
    }
}

impl Located<&NamedFunction> {
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

    /// # Convert this located named function to a located function
    pub fn as_located_function(&self) -> Located<&Function> {
        Located {
            fragment: &self.fragment.inner,
            location: FunctionLocation::NamedFunction {
                index: self.location,
            },
        }
    }
}

impl Located<&Function> {
    /// # Iterate over the function's branches
    pub fn branches(&self) -> impl Iterator<Item = Located<&Branch>> {
        let function = self.fragment;
        let location = self.location.clone();

        function
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
    pub fn find_single_branch(&self) -> Option<Located<&Branch>> {
        let function = &self.fragment;
        let location = self.location.clone();

        if function.branches.len() > 1 {
            return None;
        }

        function
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

impl<'r> Located<&'r mut Function> {
    /// # Destructure the located function into its component parts
    ///
    /// Unfortunately, following the pattern set by the `Located<&Function>` API
    /// doesn't work here, due to lifetime issues.
    pub fn destructure(
        self,
    ) -> (Vec<Located<&'r mut Branch>>, &'r mut BTreeSet<String>) {
        let branches = self
            .fragment
            .branches
            .iter_mut()
            .map(|(&index, branch)| Located {
                fragment: branch,
                location: BranchLocation {
                    parent: Box::new(self.location.clone()),
                    index,
                },
            })
            .collect();
        let environment = &mut self.fragment.environment;

        (branches, environment)
    }
}

impl Located<&Branch> {
    /// # Iterate over the expressions in the branch's body
    pub fn body(&self) -> impl Iterator<Item = Located<&Expression>> {
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

impl<'r> Located<&'r mut Branch> {
    /// # Destructure the located function into its component parts
    ///
    /// Unfortunately, following the pattern set by the `Located<&Branch>` API
    /// doesn't work here, due to lifetime issues.
    pub fn destructure(
        self,
    ) -> (Vec<Located<&'r mut Expression>>, &'r mut Vec<Pattern>) {
        let expressions = self
            .fragment
            .body
            .iter_mut()
            .map(|(&index, branch)| Located {
                fragment: branch,
                location: ExpressionLocation {
                    parent: Box::new(self.location.clone()),
                    index,
                },
            })
            .collect();
        let parameters = &mut self.fragment.parameters;

        (expressions, parameters)
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
