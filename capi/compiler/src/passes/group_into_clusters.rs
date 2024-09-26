use crate::syntax::{Cluster, Clusters, Function};

pub fn group_into_clusters(functions: Vec<Function>) -> Clusters {
    // This is just a placeholder implementation, while support for clusters is
    // still being implemented.
    Clusters {
        clusters: functions
            .into_iter()
            .map(|function| [function])
            .map(Vec::from)
            .map(|functions| Cluster { functions })
            .collect(),
    }
}
