use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use super::{
    FragmentId, FragmentMap, Function, FunctionIndexInCluster,
    FunctionIndexInRootContext, FunctionLocation,
};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    /// # The first fragment in the root context
    ///
    /// This indirectly points to all existing fragments.
    pub root: Option<FragmentId>,

    /// # The named functions in the root context
    pub functions: BTreeMap<FunctionIndexInRootContext, Function>,

    /// # The function clusters
    pub clusters: Vec<Cluster>,

    pub map: FragmentMap,
}

impl Fragments {
    /// # Find a cluster, by the ID of a function within it
    ///
    /// Returns `None`, if the provided `FragmentId` does not refer to a named
    /// function. All named functions are grouped into clusters, so this should
    /// always return `Some` otherwise.
    pub fn find_cluster_by_function_id(
        &self,
        function_id: &FragmentId,
    ) -> Option<&Cluster> {
        self.clusters.iter().find(|cluster| {
            cluster.functions.values().any(|(id, _)| id == function_id)
        })
    }

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
            .find(|cluster| cluster.functions.values().any(|(_, i)| i == index))
    }

    /// # Find the function with the provided name
    ///
    /// ## Implementation Note
    ///
    /// There is currently a function with a similar name on `FragmentMap`. Once
    /// the ongoing refactoring on fragment addressing has finished, that
    /// function can be removed, and this one can take over its name.
    pub fn find_function_by_name2(
        &self,
        name: &str,
    ) -> Option<(&Function, FunctionLocation)> {
        self.functions.iter().find_map(|(index, function)| {
            if function.name.as_deref() == Some(name) {
                let location =
                    FunctionLocation::NamedFunction { index: *index };
                Some((function, location))
            } else {
                None
            }
        })
    }
}

impl Deref for Fragments {
    type Target = FragmentMap;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Fragments {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Cluster {
    pub functions: BTreeMap<
        FunctionIndexInCluster,
        (FragmentId, FunctionIndexInRootContext),
    >,
}
