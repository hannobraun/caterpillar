use super::Function;

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

impl Cluster {
    /// # Create a new cluster
    pub fn new(functions: Vec<Function>) -> Self {
        Self { functions }
    }

    /// # Access the functions in the cluster
    pub fn into_functions(self) -> impl Iterator<Item = Function> {
        self.functions.into_iter()
    }
}
