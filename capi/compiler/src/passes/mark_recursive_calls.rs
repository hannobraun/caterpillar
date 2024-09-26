use std::collections::BTreeSet;

use crate::syntax::{Cluster, Expression, IdentifierTarget};

pub fn mark_recursive_calls(clusters: &mut Vec<Cluster>) {
    for cluster in clusters {
        let function_names_in_cluster = cluster
            .functions
            .iter()
            .filter_map(|function| function.name.clone())
            .collect::<BTreeSet<_>>();

        for function in &mut cluster.functions {
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
