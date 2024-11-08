mod branch;
mod expression;
mod located;

pub use self::{
    branch::BranchLocation, expression::ExpressionLocation, located::Located,
};

use std::fmt;

use super::{Functions, Index, NamedFunction};

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
