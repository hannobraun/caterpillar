use super::Function;

/// # All functions in the code, grouped by clusters
pub struct Clusters {
    /// # All named functions, in the original order they were defined in
    pub functions: Vec<Function>,

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
    /// # The functions in the cluster
    pub functions: Vec<Function>,
}
