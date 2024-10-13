use crate::syntax::Cluster;

use super::{FunctionLocation, NamedFunctions};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Fragments {
    /// # The named functions in the root context
    pub named_functions: NamedFunctions,

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
}
