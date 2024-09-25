use crate::syntax::{Cluster, Function};

pub fn group_into_clusters(functions: Vec<Function>) -> Vec<Cluster> {
    // This is just a placeholder implementation, while support for clusters is
    // still being implemented.
    functions
        .into_iter()
        .map(|function| [function])
        .map(Vec::from)
        .map(Cluster::new)
        .collect()
}
