use std::collections::BTreeSet;

use capi_runtime::Value;

use crate::code::Index;

use super::{
    search::Find, BranchLocation, Cluster, Expression, ExpressionLocation,
    FunctionLocation, Hash, IndexMap,
};

/// # All named functions in a program
///
/// At this point, all named functions live in a single root context, and are
/// addressed by an index into that root context. The language is expected to
/// grow a module system in the future, and then this will change.
///
/// Additionally, functions are content-addressed, and can be referred to with a
/// hash that is expected to be unique to that function. This requires the
/// function to be fully pre-compiled (or the hash would not remain stable), but
/// is the more future-proof way of referring to functions.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Functions {
    inner: FunctionsInner,
}

impl Functions {
    /// # Insert the provided named function
    ///
    /// ## Panics
    ///
    /// Panics, if the added function does not have a name.
    pub fn insert(&mut self, function: Function) {
        assert!(
            function.name.is_some(),
            "Trying to insert named function that does not actually have a \
            name."
        );

        self.inner.push(function);
    }

    /// # Access the named function at the given index
    pub fn get(&self, index: &Index<Function>) -> Option<&Function> {
        self.inner.get(index)
    }

    /// # Access the named function at the given index mutably
    pub fn get_mut(
        &mut self,
        index: &Index<Function>,
    ) -> Option<&mut Function> {
        self.inner.get_mut(index)
    }

    /// # Find the named function with the provided hash
    pub fn find_by_hash(
        &self,
        hash: &Hash<Function>,
    ) -> Option<Find<&Function, Index<Function>>> {
        self.inner.iter().find_map(|(&index, function)| {
            if &Hash::new(function) == hash {
                Some(Find {
                    find: function,
                    metadata: index,
                })
            } else {
                None
            }
        })
    }

    /// # Find the named function with the provided index
    pub fn find_by_index(
        &self,
        index: &Index<Function>,
    ) -> Option<Find<&Function, Index<Function>>> {
        let function = self.inner.get(index)?;
        Some(Find {
            find: function,
            metadata: *index,
        })
    }

    /// # Find the function with the provided name
    pub fn find_by_name(
        &self,
        name: &str,
    ) -> Option<Find<&Function, Index<Function>>> {
        self.inner.iter().find_map(|(&index, function)| {
            if function.name.as_deref() == Some(name) {
                Some(Find {
                    find: function,
                    metadata: index,
                })
            } else {
                None
            }
        })
    }

    /// # Find the branch at the given location
    pub fn find_branch_by_location(
        &self,
        location: &BranchLocation,
    ) -> Option<&Branch> {
        let function = self.find_function_by_location(&location.parent)?;
        function.branches.get(&location.index)
    }

    /// # Find the expression at the given location
    pub fn find_expression_by_location(
        &self,
        location: &ExpressionLocation,
    ) -> Option<&Expression> {
        let branch = self.find_branch_by_location(&location.parent)?;
        branch.body.get(&location.index)
    }

    /// # Find the function at the given location
    ///
    /// This includes both named and anonymous functions.
    pub fn find_function_by_location(
        &self,
        location: &FunctionLocation,
    ) -> Option<&Function> {
        match location {
            FunctionLocation::NamedFunction { index } => self.inner.get(index),
            FunctionLocation::AnonymousFunction { location } => {
                let expression = self.find_expression_by_location(location)?;
                expression.as_function()
            }
        }
    }

    /// # Iterate over the named functions
    pub fn functions(&self) -> impl Iterator<Item = &Function> {
        self.inner.values()
    }

    /// # Iterate over the named functions mutably
    pub fn functions_mut(&mut self) -> impl Iterator<Item = &mut Function> {
        self.inner.values_mut()
    }

    /// # Consume this instance and return an iterator over the functions
    pub fn into_named_functions(self) -> impl Iterator<Item = Function> {
        self.inner.into_values()
    }
}

impl IntoIterator for Functions {
    type Item = <FunctionsInner as IntoIterator>::Item;
    type IntoIter = <FunctionsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'r> IntoIterator for &'r Functions {
    type Item = <&'r FunctionsInner as IntoIterator>::Item;
    type IntoIter = <&'r FunctionsInner as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&self.inner).into_iter()
    }
}

type FunctionsInner = IndexMap<Function>;

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
    /// # The name of this function, if available
    ///
    /// A name is not available for anonymous functions.
    ///
    /// ## Implementation Note
    ///
    /// This happens to work for now, but it is a stopgap. It makes more sense
    /// to associate a name with a function where it is defined. As of this
    /// writing, this would be the root scope for all named functions. In the
    /// future, it could be any module.
    ///
    /// This would also allow supporting function aliases. Right now, these
    /// would break the assumption that is encoded here, that all functions have
    /// at most one name.
    pub name: Option<String>,

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

    /// # The index of this function within its cluster
    ///
    /// This is defined for named functions only. The value is `None` for
    /// anonymous functions.
    pub index_in_cluster: Option<Index<(Function, Cluster)>>,
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
