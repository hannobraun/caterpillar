use std::iter;

use crate::{
    code::{
        syntax::{
            Function, FunctionLocation, Located, NamedFunction, SyntaxTree,
        },
        FunctionCalls, Functions, Index, IndexMap,
    },
    passes::resolve_dependencies,
};

/// # Tracks the dependencies between functions
///
/// A dependency can take two forms:
///
/// - If any function `f` calls a named function `g`, then `f` depends on `g`.
/// - If any function `f` has a local function `g`, then `f` depends on `g`.
///
/// Dependencies form a graph, not a tree, since:
///
/// - Named functions can call themselves.
/// - Named functions can call each other.
/// - Local functions can call their top-level parent function (which is always
///   named), or another named function that (directly or indirectly) calls
///   their top-level parent function.
///
/// Named functions that call themselves are designated as "self-recursive".
/// Groups of mutually dependent functions are designated as "mutually
/// recursive".
///
/// All functions are grouped into [`Cluster`]s. Either by themselves, if they
/// are nor recursive or self-recursive, or together with all the functions in
/// their mutually dependent group.
///
/// Using graph theory jargon, dependency clusters are a "condensation" of the
/// original dependency graph.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Dependencies {
    clusters: Vec<Cluster>,
}

impl Dependencies {
    /// # Resolve the dependencies
    pub fn resolve(
        syntax_tree: &SyntaxTree,
        function_calls: &FunctionCalls,
    ) -> Self {
        let clusters = resolve_dependencies(syntax_tree, function_calls);
        Self { clusters }
    }

    /// # Iterate over the function clusters, from the leaves up
    ///
    /// Guarantees that any cluster that is yielded by the iterator only has
    /// dependencies on functions in clusters that have already been yielded
    /// before.
    pub fn clusters_from_leaves(&self) -> impl Iterator<Item = &Cluster> {
        self.clusters.iter().rev()
    }

    /// # Iterate over the function clusters, from the leaves up, mutably
    ///
    /// The documented behavior of [`Self::clusters_from_leaves`] applies to
    /// this method too.
    pub fn clusters_from_leaves_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut Cluster> {
        self.clusters.iter_mut().rev()
    }

    /// # Iterate over all functions, from the leaves up
    ///
    /// Guarantees that any function that is yielded by the iterator only has
    /// dependencies on functions that have already been yielded before.
    pub fn functions_from_leaves(
        &self,
    ) -> impl Iterator<Item = (&FunctionLocation, &Cluster)> {
        self.clusters_from_leaves().flat_map(|cluster| {
            cluster.functions.values().zip(iter::repeat(cluster))
        })
    }

    /// # Find the cluster containing a given named function
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
/// See [`OrderedFunctions`] for more information.
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
}

impl Cluster {
    /// # Iterate over the functions in the cluster
    ///
    /// ## Panics
    ///
    /// Panics, if the provided [`Functions`] instance does not contain a
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

    /// # Find the function with the given location in the cluster
    pub fn find_function_by_location(
        &self,
        location: &FunctionLocation,
    ) -> Option<Index<FunctionLocation>> {
        self.functions.iter().find_map(|(&index, l)| {
            if location == l {
                Some(index)
            } else {
                None
            }
        })
    }
}
