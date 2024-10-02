use std::{collections::BTreeMap, iter};

use crate::{
    fragments::FunctionIndexInCluster,
    syntax::{Cluster, Clusters, Function, NamedFunctionIndex},
};

pub fn group_into_clusters(functions: Vec<Function>) -> Clusters {
    // This is just a placeholder implementation, while support for clusters is
    // still being implemented.

    let clusters = iter::successors(Some(0), |i| Some(i + 1))
        .take(functions.len())
        .map(|i| Cluster {
            functions: BTreeMap::from([(
                FunctionIndexInCluster(0),
                NamedFunctionIndex(i),
            )]),
        })
        .collect();
    let functions = {
        let indices = iter::successors(Some(0), |i| Some(i + 1));
        indices
            .zip(functions)
            .map(|(index, function)| (NamedFunctionIndex(index), function))
            .collect()
    };

    Clusters {
        functions,
        clusters,
    }
}
