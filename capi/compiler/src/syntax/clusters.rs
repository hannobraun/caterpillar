use std::collections::BTreeMap;

use crate::fragments::{FunctionIndexInCluster, FunctionIndexInRootContext};

use super::Function;

/// # All functions in the code, grouped by clusters
#[derive(Debug)]
pub struct Clusters {
    /// # All named functions, in the original order they were defined in
    pub functions: BTreeMap<FunctionIndexInRootContext, Function>,

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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Cluster {
    /// # Indices that refer to the functions in the cluster
    ///
    /// The indices refer to the functions in their original order within the
    /// list of all named functions.
    pub functions: BTreeMap<FunctionIndexInCluster, FunctionIndexInRootContext>,
}
