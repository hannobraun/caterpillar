use std::collections::{BTreeMap, BTreeSet};

use crate::{hash::Hash, syntax::Pattern};

use super::{
    search::FoundFunction, BranchIndex, BranchLocation, Fragment,
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
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct NamedFunctions {
    pub inner: BTreeMap<FunctionIndexInRootContext, Function>,
}

impl NamedFunctions {
    /// # Find the named function with the provided hash
    pub fn find_named_function_by_hash(
        &self,
        hash: &Hash<Function>,
    ) -> Option<FoundFunction> {
        self.inner.iter().find_map(|(&index, function)| {
            if &Hash::new(function) == hash {
                Some(FoundFunction {
                    function: function.clone(),
                    location: FunctionLocation::NamedFunction { index },
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
