use std::iter;

use crate::syntax::{Cluster, Clusters, Function};

pub fn group_into_clusters(functions: Vec<Function>) -> Clusters {
    // This is just a placeholder implementation, while support for clusters is
    // still being implemented.
    let clusters = iter::successors(Some(0), |i| Some(i + 1))
        .take(functions.len())
        .map(|i| Cluster { functions: vec![i] })
        .collect();
    let functions = functions.into_iter().enumerate().collect();
    Clusters {
        functions,
        clusters,
    }
}
