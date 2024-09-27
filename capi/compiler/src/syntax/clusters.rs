use std::collections::BTreeMap;

use super::Function;

/// # All functions in the code, grouped by clusters
pub struct Clusters {
    /// # All named functions, in the original order they were defined in
    pub functions: BTreeMap<NamedFunctionIndex, Function>,

    /// # The named functions, grouped into clusters
    pub clusters: Vec<Cluster>,
}

/// # A cluster of functions
///
/// During compilation, all functions are grouped into clusters. A cluster can
/// consist of a single function, or a group of mutually recursive functions.
///
/// All mutually recursive functions are grouped into a single clusters with the
/// other functions in their recursive group.
pub struct Cluster {
    /// # Indices that refer to the functions in the cluster
    ///
    /// The indices refer to the functions in their original order within the
    /// list of all named functions.
    pub functions: BTreeMap<usize, NamedFunctionIndex>,
}

/// # An index into the list of all named functions
///
/// Assumes named functions are ordered as they appear in the source code.
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub struct NamedFunctionIndex(pub u32);
