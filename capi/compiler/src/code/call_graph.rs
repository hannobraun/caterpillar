use std::{collections::BTreeSet, iter};

use super::{
    search::Find, BranchLocation, Function, Index, IndexMap, NamedFunctions,
};

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

    /// # Iterate over the function clusters, from the leaves up, mutably
    ///
    /// Guarantees that any cluster that is yielded by the iterator only has
    /// non-recursive calls to functions in clusters that have already been
    /// yielded before.
    pub fn clusters_from_leaves_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Cluster> {
        self.clusters.iter_mut().rev()
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

    /// # The functions in this cluster that never terminate (diverge)
    ///
    /// Any functions that are _not_ in this set might still never terminate at
    /// runtime. We can't know, that's called the halting problem.
    ///
    /// But what we do know, is that any function in this set is _guaranteed_ to
    /// never terminate.
    ///
    /// Starts out as `None`, as the compiler pass that creates the call graph
    /// (and those all [`Cluster`]s does not have the information required to do
    /// this analysis. It is later filled in by another compiler pass.
    pub divergent_functions: Option<BTreeSet<Index<Function>>>,

    /// # The branches in this cluster that are _not_ diverging
    ///
    /// Contains any branches in this cluster from functions that are _not_
    /// known to diverge. The branches are sorted topologically, meaning any
    /// branch in this list is guaranteed to only call branches that come later
    /// in the list.
    ///
    /// Starts out as `None`, as the compiler pass that creates the call graph
    /// (and those all [`Cluster`]s does not have the information required to do
    /// this analysis. It is later filled in by another compiler pass.
    pub non_divergent_branches: Option<Vec<BranchLocation>>,
}

impl Cluster {
    /// # Iterate over the functions in the cluster
    ///
    /// ## Panics
    ///
    /// Panics, if the provided [`NamedFunctions`] instance does not contain a
    /// function referenced in this cluster. Unless you're mixing data
    /// structures from different compiler passes, this should never happen. If
    /// it still does, that's a bug.
    pub fn functions<'r>(
        &'r self,
        named_functions: &'r NamedFunctions,
    ) -> impl Iterator<Item = Find<&'r Function, Index<Function>>> + 'r {
        self.functions.values().copied().map(|index| {
            named_functions
                .find_by_index(&index)
                .expect("Function referred to from cluster must exist.")
        })
    }
}
