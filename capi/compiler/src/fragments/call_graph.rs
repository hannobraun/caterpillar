use crate::syntax::Cluster;

use super::FunctionLocation;

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
        location: &FunctionLocation,
    ) -> Option<&Cluster> {
        let FunctionLocation::NamedFunction { index } = location else {
            panic!("Can't search for cluster by anonymous function.");
        };

        self.clusters
            .iter()
            .find(|cluster| cluster.functions.values().any(|i| i == index))
    }
}
