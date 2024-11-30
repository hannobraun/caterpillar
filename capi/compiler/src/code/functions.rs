use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use capi_runtime::Value;

use crate::code::Index;

use super::{
    BranchLocation, Expression, ExpressionLocation, FunctionLocation, IndexMap,
    Located,
};

/// # All functions in the program
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Functions {
    /// # The named functions
    pub named: NamedFunctions,

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
        branch.body.get(&location.index)
    }

    /// # Iterate over all functions, both named and anonymous
    pub fn all_functions(&self) -> impl Iterator<Item = Located<&Function>> {
        self.inner.iter().map(|(location, function)| Located {
            fragment: function,
            location: location.clone(),
        })
    }
}

/// # The named functions in the program
///
/// At this point, all named functions live in a single root context, and are
/// addressed by an index into that root context. The language is expected to
/// grow a module system in the future, and then this will change.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct NamedFunctions {
    inner: IndexMap<NamedFunction>,
}

impl NamedFunctions {
    /// # Access the named function with the provided index
    ///
    /// Returns `None`, if the there is no named function at the provided index.
    pub fn by_index(
        &self,
        index: &Index<NamedFunction>,
    ) -> Option<Located<&NamedFunction>> {
        let function = self.inner.get(index)?;

        Some(Located {
            fragment: function,
            location: *index,
        })
    }

    /// # Access the named parent function for the given function, if anonymous
    ///
    /// If the location of a named function is provided, that named function
    /// itself is returned.
    ///
    /// Returns `None`, if no parent named function can be found.
    pub fn by_child_function(
        &self,
        location: &FunctionLocation,
    ) -> Option<Located<&NamedFunction>> {
        let index = match location {
            FunctionLocation::NamedFunction { index } => index,
            FunctionLocation::AnonymousFunction { location } => {
                return self.by_child_function(&location.parent.parent);
            }
        };

        let named_function = self.inner.get(index)?;

        Some(Located {
            fragment: named_function,
            location: *index,
        })
    }

    /// # Iterate over the named functions
    pub fn iter(&self) -> impl Iterator<Item = Located<&NamedFunction>> {
        self.inner.iter().map(|(index, named_function)| Located {
            fragment: named_function,
            location: *index,
        })
    }
}

impl Deref for NamedFunctions {
    type Target = IndexMap<NamedFunction>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for NamedFunctions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
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
    pub body: IndexMap<Expression>,
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
