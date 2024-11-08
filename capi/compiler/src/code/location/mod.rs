mod expression;

pub use self::expression::ExpressionLocation;

use std::{fmt, ops::Deref};

use super::{Branch, Expression, Functions, Index, NamedFunction};

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
    pub fragment: T,

    /// # The additional search-specific metadata
    pub location: M,
}

impl<T, M> Deref for Located<T, M> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.fragment
    }
}

impl<F> Located<F, Index<NamedFunction>> {
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

impl<M> Located<&NamedFunction, M>
where
    M: Clone + Into<FunctionLocation>,
{
    /// # Iterate over the function's branches
    pub fn branches(
        &self,
    ) -> impl Iterator<Item = Located<Branch, BranchLocation>> {
        let function = &self.fragment;
        let location = self.location.clone().into();

        function.inner.branches.clone().into_iter().map(
            move |(index, branch)| Located {
                fragment: branch,
                location: BranchLocation {
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
                fragment: branch.clone(),
                location: BranchLocation {
                    parent: Box::new(location),
                    index,
                },
            })
    }
}

impl Located<Branch, BranchLocation> {
    /// # Iterate over the expressions in the branch's body
    pub fn body(
        &self,
    ) -> impl Iterator<Item = Located<Expression, ExpressionLocation>> {
        let location = self.location.clone();
        self.body
            .clone()
            .into_iter()
            .map(move |(index, expression)| Located {
                fragment: expression,
                location: ExpressionLocation {
                    parent: Box::new(location.clone()),
                    index,
                },
            })
    }
}

/// # The location of a branch in the source code
#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct BranchLocation {
    pub parent: Box<FunctionLocation>,
    pub index: Index<Branch>,
}

impl BranchLocation {
    /// # Create a helper that implements [`fmt::Display`]
    pub fn display<'r>(
        &'r self,
        functions: &'r Functions,
    ) -> BranchLocationDisplay<'r> {
        BranchLocationDisplay {
            location: self,
            functions,
        }
    }
}

/// # Helper struct to display [`BranchLocation`]
///
/// Implements [`fmt::Display`], which [`BranchLocation`] itself doesn't.
pub struct BranchLocationDisplay<'r> {
    location: &'r BranchLocation,
    functions: &'r Functions,
}

impl fmt::Display for BranchLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "branch {} of {}",
            self.location.index,
            self.location.parent.display(self.functions),
        )
    }
}

/// # The location of a function in the source code
#[derive(
    Clone,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
)]
pub enum FunctionLocation {
    NamedFunction { index: Index<NamedFunction> },
    AnonymousFunction { location: ExpressionLocation },
}

impl FunctionLocation {
    /// # Create a helper that implements [`fmt::Display`]
    pub fn display<'r>(
        &'r self,
        functions: &'r Functions,
    ) -> FunctionLocationDisplay<'r> {
        FunctionLocationDisplay {
            location: self,
            functions,
        }
    }
}

impl From<Index<NamedFunction>> for FunctionLocation {
    fn from(index: Index<NamedFunction>) -> Self {
        Self::NamedFunction { index }
    }
}

/// # Helper struct to display [`FunctionLocation`]
///
/// Implements [`fmt::Display`], which [`FunctionLocation`] itself doesn't.
pub struct FunctionLocationDisplay<'r> {
    location: &'r FunctionLocation,
    functions: &'r Functions,
}

impl fmt::Display for FunctionLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.location {
            FunctionLocation::NamedFunction { index } => {
                let name = &self
                    .functions
                    .find_named_by_index(index)
                    .expect("Named function referred to be index must exist")
                    .name;

                write!(f, "named function `{name}`")?;
            }
            FunctionLocation::AnonymousFunction { location } => {
                write!(
                    f,
                    "anonymous function at {}",
                    location.display(self.functions),
                )?;
            }
        }

        Ok(())
    }
}
