use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use crate::syntax;

use super::{
    FragmentId, FragmentMap, Function, FunctionIndexInCluster,
    FunctionIndexInRootContext,
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

    /// # The named functions, collected by clusters with recursive calls within
    ///
    /// ## Implementation Note
    ///
    /// As of this writing, there is an ongoing effort to simplify the code
    /// representation. This field is part of that effort. In contrast to the
    /// `clusters` field, it doesn't refer to functions by ID, but by per index
    /// into the `functions` field.
    ///
    /// If this simplification effort is successful, the `clusters` field is
    /// going to be removed, and the name of this field can take over its name.
    pub clusters2: Vec<syntax::Cluster>,

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
            cluster.functions.values().any(|id| id == function_id)
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
    pub functions: BTreeMap<FunctionIndexInCluster, FragmentId>,
}
