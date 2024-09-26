use std::collections::BTreeMap;

use crate::syntax::{Clusters, Expression, IdentifierTarget};

pub fn mark_recursive_calls(clusters: &mut Clusters) {
    for cluster in &mut clusters.clusters {
        let function_names_in_cluster = cluster
            .functions
            .iter()
            .copied()
            .filter_map(|index| {
                clusters.functions[index]
                    .name
                    .clone()
                    .map(|name| (name, index))
            })
            .collect::<BTreeMap<_, _>>();

        for &index in &cluster.functions {
            let function = &mut clusters.functions[index];

            for branch in &mut function.branches {
                for expression in &mut branch.body {
                    if let Expression::Identifier {
                        name,
                        target:
                            Some(IdentifierTarget::Function {
                                is_known_to_be_recursive_call_to_index,
                            }),
                        ..
                    } = expression
                    {
                        if let Some(index) = function_names_in_cluster.get(name)
                        {
                            let index: u32 = (*index)
                                .try_into()
                                .expect("Expecting to run on 32-bit platform.");

                            *is_known_to_be_recursive_call_to_index =
                                Some(index);
                        }
                    }
                }
            }
        }
    }
}
