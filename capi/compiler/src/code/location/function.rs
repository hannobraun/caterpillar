use std::fmt;

use crate::code::{syntax::SyntaxTree, Branch, Function, Index, NamedFunction};

use super::{
    located::HasLocation, BranchLocation, ExpressionLocation, Located,
};

impl HasLocation for Function {
    type Location = FunctionLocation;
}

impl<'r> Located<&'r Function> {
    /// # Iterate over the function's branches
    pub fn branches(self) -> impl Iterator<Item = Located<&'r Branch>> {
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
    pub fn destructure(self) -> Vec<Located<&'r mut Branch>> {
        self.fragment
            .branches
            .iter_mut()
            .map(|(&index, branch)| Located {
                fragment: branch,
                location: BranchLocation {
                    parent: Box::new(self.location.clone()),
                    index,
                },
            })
            .collect()
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
        syntax_tree: &'r SyntaxTree,
    ) -> FunctionLocationDisplay<'r> {
        FunctionLocationDisplay {
            location: self,
            syntax_tree,
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
    syntax_tree: &'r SyntaxTree,
}

impl fmt::Display for FunctionLocationDisplay<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.location {
            FunctionLocation::NamedFunction { index } => {
                let name = &self
                    .syntax_tree
                    .named_functions
                    .get(index)
                    .expect("Named function referred to be index must exist")
                    .name;

                write!(f, "named function `{name}`")?;
            }
            FunctionLocation::AnonymousFunction { location } => {
                write!(
                    f,
                    "anonymous function at {}",
                    location.display(self.syntax_tree),
                )?;
            }
        }

        Ok(())
    }
}
