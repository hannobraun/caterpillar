use std::collections::BTreeMap;

use super::{FunctionIndexInCluster, FunctionIndexInRootContext};

/// # The program's named functions, organized as a call graph
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CallGraph {
    clusters: Vec<Cluster>,
}

impl CallGraph {
    /// # Insert a cluster
    ///
    /// ## Implementation Note
    ///
    /// This method is way too low-level and does nothing to help with anything
    /// related to a call graph. Its existence is an artifact of the ongoing
    /// compiler cleanup.
    pub fn insert(&mut self, cluster: Cluster) {
        self.clusters.push(cluster);
    }

    /// # Iterate over all named functions, from the leaves up
    ///
    /// Guarantees that any function that is yielded by the iterator only has
    /// non-recursive calls to functions that have already been yielded before.
    pub fn functions_from_leaves(
        &self,
    ) -> impl Iterator<Item = &FunctionIndexInRootContext> {
        self.clusters
            .iter()
            .rev()
            .flat_map(|cluster| cluster.functions.values())
    }

    /// # Find the cluster containing a given function
    ///
    /// ## Panics
    ///
    /// Panics, if the provided location does not refer to a named function.
    pub fn find_cluster_by_named_function(
        &self,
        index: &FunctionIndexInRootContext,
    ) -> Option<&Cluster> {
        self.clusters
            .iter()
            .find(|cluster| cluster.functions.values().any(|i| i == index))
    }
}

/// # A cluster of functions
///
/// During compilation, all functions are grouped into clusters. A cluster can
/// consist of a single function, or a group of mutually recursive functions.
///
/// All mutually recursive functions are grouped into a single clusters with the
/// other functions in their recursive group.
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
pub struct Cluster {
    /// # Indices that refer to the functions in the cluster
    ///
    /// The indices refer to the functions in their original order within the
    /// list of all named functions.
    pub functions: BTreeMap<FunctionIndexInCluster, FunctionIndexInRootContext>,
}
