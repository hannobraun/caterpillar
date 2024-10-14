use std::collections::{BTreeMap, BTreeSet};

use capi_runtime::Value;

use crate::hash::Hash;

use super::{
    search::Find, BranchIndex, BranchLocation, Fragment,
    FragmentIndexInBranchBody, FragmentLocation, FunctionIndexInCluster,
    FunctionIndexInRootContext, FunctionLocation,
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
pub struct NamedFunctions {
    inner: NamedFunctionsInner,
}

impl NamedFunctions {
    /// # Insert the provided named function
    ///
    /// ## Implementation Note
    ///
    /// The signature of this function doesn't make a whole lot of sense. The
    /// index should be created within this function and returned from it.
    ///
    /// This is an artifact of the ongoing compiler cleanup. The change to the
    /// signature can't be made until `NamedFunctions` is used earlier in the
    /// compiler pipeline, which is blocked until `syntax::Function` and
    /// `fragments::Function` are merged.
    pub fn insert(
        &mut self,
        index: FunctionIndexInRootContext,
        function: Function,
    ) {
        assert!(
            self.get(&index).is_none(),
            "Should not overwrite named function."
        );
        assert!(
            function.name.is_some(),
            "Trying to insert named function that does not actually have a \
            name."
        );

        self.inner.insert(index, function);
    }

    /// # Access the named function at the given index
    pub fn get(&self, index: &FunctionIndexInRootContext) -> Option<&Function> {
        self.inner.get(index)
    }

    /// # Find the named function with the provided hash
    pub fn find_by_hash(
        &self,
        hash: &Hash<Function>,
    ) -> Option<Find<Function, FunctionLocation>> {
        self.inner.iter().find_map(|(&index, function)| {
            if &Hash::new(function) == hash {
                Some(Find {
                    find: function.clone(),
                    metadata: FunctionLocation::NamedFunction { index },
                })
            } else {
                None
            }
        })
    }

    /// # Find the function with the provided name
    pub fn find_by_name(
        &self,
        name: &str,
    ) -> Option<Find<Function, FunctionIndexInRootContext>> {
        self.inner.iter().find_map(|(&index, function)| {
            if function.name.as_deref() == Some(name) {
                Some(Find {
                    find: function.clone(),
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

    /// # Find the fragment at the given location
    pub fn find_fragment_by_location(
        &self,
        location: &FragmentLocation,
    ) -> Option<&Fragment> {
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
                let fragment = self.find_fragment_by_location(location)?;
                fragment.as_function()
            }
        }
    }

    /// # Iterate over the named functions
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = (&FunctionIndexInRootContext, &Function)> {
        self.inner.iter()
    }
}

impl IntoIterator for NamedFunctions {
    type Item = <NamedFunctionsInner as IntoIterator>::Item;
    type IntoIter = <NamedFunctionsInner as IntoIterator>::IntoIter;

    /// # Convert this struct into an iterator over the named functions
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

type NamedFunctionsInner = BTreeMap<FunctionIndexInRootContext, Function>;

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
    pub branches: BTreeMap<BranchIndex, Branch>,

    /// # Values captured by the function from a parent scope
    ///
    /// All functions in Caterpillar are closures that can use values from
    /// parent scopes. The names of those values are stored here.
    pub environment: BTreeSet<String>,

    /// # The index of this function within its cluster
    ///
    /// This is defined for named functions only. The value is `None` for
    /// anonymous functions.
    pub index_in_cluster: Option<FunctionIndexInCluster>,
}

impl Function {
    /// # Expect the function to have one branch and access that
    ///
    /// This is a convenience method, designed for tests and such. It should not
    /// be used in code that requires proper error handling.
    ///
    /// ## Panics
    ///
    /// Panics, if the function does not have exactly one branch.
    pub fn expect_one_branch(&self) -> &Branch {
        assert_eq!(
            self.branches.len(),
            1,
            "Expected function to have exactly one branch."
        );

        self.branches
            .first_key_value()
            .map(|(_index, branch)| branch)
            .expect("Just checked that there is exactly one branch.")
    }
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
pub struct Branch {
    pub parameters: Parameters,

    /// # The body of the branch
    pub body: BTreeMap<FragmentIndexInBranchBody, Fragment>,
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
pub struct Parameters {
    pub inner: Vec<Pattern>,
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
