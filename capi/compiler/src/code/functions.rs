use std::{
    collections::{BTreeMap, BTreeSet},
    ops::Deref,
};

use capi_runtime::Value;

use crate::code::Index;

use super::{
    BranchLocation, Expression, ExpressionLocation, FunctionLocation, Hash,
    IndexMap, Located,
};

/// # All functions in the program
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Functions {
    /// # The named functions
    pub named: NamedFunctions,

    /// # The anonymous functions
    ///
    /// Anonymous functions are defined within named functions (or recursively,
    /// within other anonymous functions). They are identified by their address
    /// in the code.
    pub anonymous: BTreeMap<ExpressionLocation, Function>,
}

impl Functions {
    /// # Access the anonymous function defined at the provided location
    ///
    /// Returns `None`, if no anonymous function is defined at the given
    /// location.
    pub fn find_anonymous_by_location(
        &self,
        location: &ExpressionLocation,
    ) -> Option<Located<&Function>> {
        self.anonymous.iter().find_map(|(loc, function)| {
            if loc == location {
                Some(Located {
                    fragment: function,
                    location: FunctionLocation::AnonymousFunction {
                        location: location.clone(),
                    },
                })
            } else {
                None
            }
        })
    }

    /// # Access the function at the given location
    ///
    /// This includes both named and anonymous functions.
    ///
    /// Returns `None`, if the given location does not identify a function.
    pub fn by_location(
        &self,
        location: &FunctionLocation,
    ) -> Option<&Function> {
        match location {
            FunctionLocation::NamedFunction { index } => {
                self.named.inner.get(index).map(|function| &function.inner)
            }
            FunctionLocation::AnonymousFunction { location } => {
                self.anonymous.get(location)
            }
        }
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
        self.named
            .inner
            .iter()
            .map(|(&index, named_function)| Located {
                fragment: &named_function.inner,
                location: FunctionLocation::NamedFunction { index },
            })
            .chain(self.anonymous.iter().map(|(location, function)| Located {
                fragment: function,
                location: FunctionLocation::AnonymousFunction {
                    location: location.clone(),
                },
            }))
    }

    /// # Iterate over all functions, both named and anonymous, mutably
    pub fn all_functions_mut(
        &mut self,
    ) -> impl Iterator<Item = Located<&mut Function>> {
        self.named
            .inner
            .iter_mut()
            .map(|(&index, named_function)| Located {
                fragment: &mut named_function.inner,
                location: FunctionLocation::NamedFunction { index },
            })
            .chain(self.anonymous.iter_mut().map(|(location, function)| {
                Located {
                    fragment: function,
                    location: FunctionLocation::AnonymousFunction {
                        location: location.clone(),
                    },
                }
            }))
    }
}

/// # The named functions in the program
///
/// At this point, all named functions live in a single root context, and are
/// addressed by an index into that root context. The language is expected to
/// grow a module system in the future, and then this will change.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct NamedFunctions {
    pub inner: IndexMap<NamedFunction>,
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

    /// # Access the function with the provided name
    ///
    /// Returns `None`, if no function has the provided name.
    pub fn by_name(&self, name: &str) -> Option<Located<&NamedFunction>> {
        self.inner.iter().find_map(|(&index, function)| {
            if function.name == name {
                Some(Located {
                    fragment: function,
                    location: index,
                })
            } else {
                None
            }
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

    /// # Iterate over the named functions mutably
    pub fn iter_mut(
        &mut self,
    ) -> impl Iterator<Item = Located<&mut NamedFunction>> {
        self.inner
            .iter_mut()
            .map(|(index, named_function)| Located {
                fragment: named_function,
                location: *index,
            })
    }

    /// # Convert this instance into an iterator over the named functions
    pub fn into_iter(self) -> impl Iterator<Item = Located<NamedFunction>> {
        self.inner
            .into_iter()
            .map(|(index, named_function)| Located {
                fragment: named_function,
                location: index,
            })
    }
}

impl Deref for NamedFunctions {
    type Target = IndexMap<NamedFunction>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// # All functions in a program, stable and content-addressable
///
/// Functions can be addressed via a hash, and this is done in function calls,
/// for example. This must be done with care though, as any change to a function
/// will also change its hash, invalidating any pre-existing hashes.
///
/// This is a wrapper around [`Functions`] (it [`Deref`]s to [`Functions`]),
/// which only allows immutable access, making sure that functions (and
/// therefore their hashes) remain stable.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct StableFunctions {
    inner: Functions,
}

impl StableFunctions {
    /// # Access the named function with the provided hash
    ///
    /// This method is only available on [`StableFunctions`], to prevent callers
    /// from erroneously relying on the hash of a function that might still
    /// change. Which would be possible, if an equivalent method was available
    /// on [`Functions`] or [`NamedFunctions`].
    ///
    /// Returns `None`, if a named function with the provided hash does not
    /// exist.
    pub fn named_by_hash(
        &self,
        hash: &Hash<Function>,
    ) -> Option<Located<&NamedFunction>> {
        self.named.inner.iter().find_map(|(&index, function)| {
            if &Hash::new(&function.inner) == hash {
                Some(Located {
                    fragment: function,
                    location: index,
                })
            } else {
                None
            }
        })
    }
}

impl Deref for StableFunctions {
    type Target = Functions;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<Functions> for StableFunctions {
    fn from(functions: Functions) -> Self {
        Self { inner: functions }
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

    /// # Values captured by the function from a parent scope
    ///
    /// All functions in Caterpillar are closures that can use values from
    /// parent scopes. The names of those values are stored here.
    pub environment: BTreeSet<String>,
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
