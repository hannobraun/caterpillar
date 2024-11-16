use std::{collections::BTreeSet, iter};

use super::{
    BranchLocation, Function, FunctionLocation, Functions, Index, IndexMap,
    Located, NamedFunction,
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
    ) -> impl Iterator<Item = (&FunctionLocation, &Cluster)> {
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
        index: &Index<NamedFunction>,
    ) -> Option<&Cluster> {
        self.clusters.iter().find(|cluster| {
            cluster.functions.values().any(|location| {
                *location == FunctionLocation::NamedFunction { index: *index }
            })
        })
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
    pub functions: IndexMap<FunctionLocation>,

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
    pub divergent_functions: Option<BTreeSet<FunctionLocation>>,

    /// # The branches in this cluster that are _not_ divergent
    ///
    /// Contains any branches in this cluster from functions that are _not_
    /// known to diverge. Those are the branches from all the functions that are
    /// not tracked in `divergent_functions`.
    ///
    /// The branches are arrange such, that they can be processed by the type
    /// inference algorithm in order. The first branch will have no recursive
    /// function calls. If no such branch exists, this list is empty, and all
    /// functions in the cluster are divergent.
    ///
    /// Every following branch in the list will only have recursive calls to
    /// functions with at least one branch that came before in the list. This
    /// makes it possible to infer the types for all functions in order.
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
        functions: &'r Functions,
    ) -> impl Iterator<Item = Located<&'r Function>> + 'r {
        self.functions.values().map(|location| {
            functions
                .by_location(location)
                .expect("Function referred to from cluster must exist.")
        })
    }
}
