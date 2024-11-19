use std::{collections::BTreeSet, fmt};

use crate::code::{Branch, Function, Functions, Index, NamedFunction};

use super::{
    located::HasLocation, BranchLocation, ExpressionLocation, Located,
};

impl HasLocation for Function {
    type Location = FunctionLocation;
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

impl From<ExpressionLocation> for FunctionLocation {
    fn from(location: ExpressionLocation) -> Self {
        Self::AnonymousFunction { location }
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
                    .named
                    .by_index(index)
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
