use crate::{hash::Hash, syntax::Cluster};

use super::{
    search::FoundFunction, Branch, BranchLocation, Fragment, FragmentLocation,
    Function, FunctionLocation, NamedFunctions,
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    /// # The named functions in the root context
    pub functions: NamedFunctions,

    /// # The function clusters
    pub clusters: Vec<Cluster>,
}

impl Fragments {
    /// # Find the cluster containing a given function
    ///
    /// ## Panics
    ///
    /// Panics, if the provided location does not refer to a named function.
    pub fn find_cluster_by_named_function(
        &self,
        location: &FunctionLocation,
    ) -> Option<&Cluster> {
        let FunctionLocation::NamedFunction { index } = location else {
            panic!("Can't search for cluster by anonymous function.");
        };

        self.clusters
            .iter()
            .find(|cluster| cluster.functions.values().any(|i| i == index))
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
            FunctionLocation::NamedFunction { index } => {
                self.functions.inner.get(index)
            }
            FunctionLocation::AnonymousFunction { location } => {
                let fragment = self.find_fragment_by_location(location)?;
                fragment.as_function()
            }
        }
    }

    /// # Find the named function with the provided hash
    pub fn find_named_function_by_hash(
        &self,
        hash: &Hash<Function>,
    ) -> Option<FoundFunction> {
        self.functions.inner.iter().find_map(|(&index, function)| {
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

    /// # Find the function with the provided name
    ///
    /// ## Implementation Note
    ///
    /// There is currently a function with a similar name on `FragmentMap`. Once
    /// the ongoing refactoring on fragment addressing has finished, that
    /// function can be removed, and this one can take over its name.
    pub fn find_function_by_name(&self, name: &str) -> Option<FoundFunction> {
        self.functions.inner.iter().find_map(|(&index, function)| {
            if function.name.as_deref() == Some(name) {
                let location = FunctionLocation::NamedFunction { index };
                Some(FoundFunction {
                    function: function.clone(),
                    location,
                })
            } else {
                None
            }
        })
    }
}
