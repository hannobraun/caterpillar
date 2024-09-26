use std::collections::BTreeSet;

use crate::syntax::{Clusters, Expression, IdentifierTarget};

pub fn mark_recursive_calls(clusters: &mut Clusters) {
    for cluster in &mut clusters.clusters {
        let function_names_in_cluster = cluster
            .functions
            .iter()
            .copied()
            .filter_map(|index| clusters.functions[index].name.clone())
            .collect::<BTreeSet<_>>();

        for &index in &cluster.functions {
            let function = &mut clusters.functions[index];

            for branch in &mut function.branches {
                for expression in &mut branch.body {
                    if let Expression::Identifier {
                        name,
                        target:
                            Some(IdentifierTarget::Function {
                                is_known_to_be_recursive,
                            }),
                        ..
                    } = expression
                    {
                        if function_names_in_cluster.contains(name) {
                            *is_known_to_be_recursive = true;
                        }
                    }
                }
            }
        }
    }
}
