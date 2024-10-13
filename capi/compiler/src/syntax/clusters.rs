use std::collections::BTreeMap;

use crate::fragments::{Cluster, FunctionIndexInRootContext};

use super::Function;

/// # All functions in the code, grouped by clusters
#[derive(Debug)]
pub struct Clusters {
    /// # All named functions, in the original order they were defined in
    pub functions: BTreeMap<FunctionIndexInRootContext, Function>,

    /// # The named functions, grouped into clusters
    pub clusters: Vec<Cluster>,
}
