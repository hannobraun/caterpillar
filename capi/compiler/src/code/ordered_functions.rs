use std::iter;

use super::{
    Function, FunctionLocation, Functions, Index, IndexMap, Located,
    NamedFunction,
};

/// # All functions, ordered by their dependencies
///
/// A dependency can take two forms:
///
/// - If a named function `f` calls a named function `g`, then `f` depends on
///   `g`.
/// - If an anonymous function `g` is defined within any function (named or
///   anonymous) `f`, then `f` depends on `g`.
///
/// Calls between named functions can be recursive, which means dependencies can
/// be recursive too. To reflect this, all functions are grouped into
/// [`Cluster`]s, in the following way:
///
/// - Any group of functions in the dependency graph, that mutually depend on
///   each other (a "strongly connected component" in graph theory jargon) are
///   grouped into the same cluster, without any other functions.
/// - Any functions that are not part of a mutually dependent group, are grouped
///   into a dedicated cluster by themselves.
///
/// Or using graph theory jargon, the clusters are a "condensation" of the
/// original dependency graph.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct OrderedFunctions {
    clusters: Vec<Cluster>,
}

impl OrderedFunctions {
    pub(crate) fn from_clusters(
        clusters: impl IntoIterator<Item = Cluster>,
    ) -> Self {
        Self {
            clusters: clusters.into_iter().collect(),
        }
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

    /// # Find the function with the given name in the cluster
    ///
    /// ## Panics
    ///
    /// Panics, if the provided [`Functions`] instance does not contain a
    /// function referenced in this cluster. Unless you're mixing data
    /// structures from different compiler passes, this should never happen. If
    /// it still does, that's a bug.
    pub fn find_function_by_name(
        &self,
        name: &str,
        functions: &Functions,
    ) -> Option<Index<FunctionLocation>> {
        self.functions
            .iter()
            .filter_map(|(&index_in_cluster, location)| {
                if let FunctionLocation::NamedFunction { index } = location {
                    let function = functions.named.by_index(index)?;
                    Some((index_in_cluster, function))
                } else {
                    None
                }
            })
            .find_map(|(index, function)| {
                if function.name == name {
                    Some(index)
                } else {
                    None
                }
            })
    }
}
