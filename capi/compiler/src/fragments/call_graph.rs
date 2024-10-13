use crate::syntax::Cluster;

use super::FunctionIndexInRootContext;

/// # The program's named functions, organized as a call graph
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CallGraph {
    pub clusters: Vec<Cluster>,
}

impl CallGraph {
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
