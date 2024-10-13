use crate::syntax::Cluster;

use super::FunctionIndexInRootContext;

/// # The program's named functions, organized as a call graph
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CallGraph {
    pub clusters: Vec<Cluster>,
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
