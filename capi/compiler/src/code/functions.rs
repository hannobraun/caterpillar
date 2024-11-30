use std::collections::BTreeMap;

use capi_runtime::Value;

use super::{
    BranchLocation, Expression, ExpressionLocation, FunctionLocation, IndexMap,
    Located, TypedExpression,
};

/// # All functions in the program
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Functions {
    /// # The functions
    pub inner: BTreeMap<FunctionLocation, Function>,
}

impl Functions {
    /// # Access the function at the given location
    ///
    /// This includes both named and anonymous functions.
    ///
    /// Returns `None`, if the given location does not identify a function.
    pub fn by_location(
        &self,
        location: &FunctionLocation,
    ) -> Option<Located<&Function>> {
        self.inner.get(location).map(|function| Located {
            fragment: function,
            location: location.clone(),
        })
    }

    /// # Access the branch at the given location
    ///
    /// Returns `None`, if the given location does not identify a branch.
    pub fn branch_by_location(
        &self,
        location: &BranchLocation,
    ) -> Option<&Branch> {
        let function = self.by_location(&location.parent)?;
        function.branches.get(&location.index)
    }

    /// # Access the expression at the given location
    ///
    /// Returns `None`, if the given location does not identify an expression.
    pub fn expression_by_location(
        &self,
        location: &ExpressionLocation,
    ) -> Option<&Expression> {
        let branch = self.branch_by_location(&location.parent)?;
        branch
            .body
            .get(&location.index)
            .map(|expression| &expression.inner)
    }

    /// # Iterate over all functions, both named and anonymous
    pub fn all_functions(&self) -> impl Iterator<Item = Located<&Function>> {
        self.inner.iter().map(|(location, function)| Located {
            fragment: function,
            location: location.clone(),
        })
    }
}

/// # A function that has a name
///
/// Named functions are defined in the top-level context. Functions that do not
/// have a name are anonymous, and are defined as literal values within other
/// functions.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct NamedFunction {
    /// # The name of the function
    pub name: String,

    /// # The function
    pub inner: Function,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct Function {
    /// # The branches of this function
    ///
    /// A function is made up of one or more branches. When a function is
    /// called, its arguments are matched against the parameters of each branch,
    /// until one branch matches. This branch is then evaluated.
    pub branches: IndexMap<Branch>,
}

#[derive(
    Clone,
    Debug,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub struct Branch {
    pub parameters: Vec<Pattern>,

    /// # The body of the branch
    pub body: IndexMap<TypedExpression>,
}

#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    serde::Deserialize,
    serde::Serialize,
    udigest::Digestable,
)]
pub enum Pattern {
    Identifier { name: String },
    Literal { value: Value },
}
