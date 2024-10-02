use std::{collections::BTreeMap, iter};

use crate::{
    fragments::FunctionIndexInCluster,
    syntax::{Cluster, Clusters, Function, NamedFunctionIndex},
};

pub fn group_into_clusters(functions: Vec<Function>) -> Clusters {
    // This is just a placeholder implementation, while support for clusters is
    // still being implemented.

    let functions = iter::successors(Some(0), |i| Some(i + 1))
        .map(NamedFunctionIndex)
        .zip(functions)
        .collect::<BTreeMap<_, _>>();
    let clusters = functions
        .keys()
        .map(|named_function_index| {
            let function_index_in_cluster = FunctionIndexInCluster(0);

            Cluster {
                functions: BTreeMap::from([(
                    function_index_in_cluster,
                    *named_function_index,
                )]),
            }
        })
        .collect();

    Clusters {
        functions,
        clusters,
    }
}
