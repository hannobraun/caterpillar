use std::iter;

use super::{Function, Index, IndexMap};

/// # The program's named functions, organized as a call graph
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CallGraph {
    clusters: Vec<Cluster>,
}

impl CallGraph {
    /// # Construct an instance of `CallGraph`
    ///
    /// Expects the provided clusters to be sorted topologically: The iterator
    /// must yield any cluster that contains calls to another cluster before it
    /// yields that other cluster.
    pub fn from_clusters(clusters: impl IntoIterator<Item = Cluster>) -> Self {
        Self {
            clusters: clusters.into_iter().collect(),
        }
    }

    /// # Iterate over the function clusters, from the leaves up
    ///
    /// Guarantees that any cluster that is yielded by the iterator only has
    /// non-recursive calls to functions in clusters that have already been
    /// yielded before.
    pub fn clusters_from_leaves(&self) -> impl Iterator<Item = &Cluster> {
        self.clusters.iter().rev()
    }

    /// # Iterate over all named functions, from the leaves up
    ///
    /// Guarantees that any function that is yielded by the iterator only has
    /// non-recursive calls to functions that have already been yielded before.
    pub fn functions_from_leaves(
        &self,
    ) -> impl Iterator<Item = (&Index<Function>, &Cluster)> {
        self.clusters_from_leaves().flat_map(|cluster| {
            cluster.functions.values().zip(iter::repeat(cluster))
        })
    }

    /// # Find the cluster containing a given function
    ///
    /// ## Panics
    ///
    /// Panics, if the provided location does not refer to a named function.
    pub fn find_cluster_by_named_function(
        &self,
        index: &Index<Function>,
    ) -> Option<&Cluster> {
        self.clusters
            .iter()
            .find(|cluster| cluster.functions.values().any(|i| i == index))
    }
}

/// # A cluster of functions
///
/// A cluster either contains a single function that might or might not be
/// recursive, or multiple mutually recursive functions. During compilation, all
/// functions are grouped into clusters.
///
/// A group of mutually recursive functions is guaranteed to be part of a single
/// cluster, without any other functions.
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
    /// # The functions in this cluster
    pub functions: IndexMap<Index<Function>>,
}
