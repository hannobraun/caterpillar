use std::{collections::BTreeMap, iter};

use crate::{
    fragments::{
        Branch, BranchIndex, Cluster, Fragment, FragmentId,
        FragmentIndexInBranchBody, FragmentMap, Fragments, FragmentsByLocation,
        Function, Parameters,
    },
    hash::{Hash, NextNeighbor, PrevNeighbor},
    syntax::{self, IdentifierTarget},
};

pub fn generate_fragments(clusters: syntax::Clusters) -> Fragments {
    let mut fragments = FragmentMap::default();
    let mut fragments_by_location = FragmentsByLocation::default();

    let compiled_functions = clusters
        .clusters
        .iter()
        .rev()
        .flat_map(|cluster| cluster.functions.values())
        .map(|index| {
            let function = clusters.functions[index].clone();
            let fragment = compile_function(
                function,
                &mut fragments,
                &mut fragments_by_location,
            );
            (index, fragment)
        })
        .collect::<Vec<_>>();

    let mut function_ids = Vec::new();
    let root = address_context(
        &compiled_functions
            .into_iter()
            .map(|(_, fragment)| fragment)
            .collect(),
        &mut function_ids,
        &mut fragments,
        &mut fragments_by_location,
    );

    let mut compiled_clusters = Vec::new();

    for cluster in clusters.clusters {
        let mut compiled_cluster = Cluster {
            functions: BTreeMap::new(),
        };

        for (function_index_in_cluster, named_function_index) in
            cluster.functions
        {
            let id = {
                let index: usize = named_function_index
                    .0
                    .try_into()
                    .expect("Expecting `usize` to be at least 32-bit.");

                function_ids[index]
            };
            compiled_cluster
                .functions
                .insert(function_index_in_cluster, id);
        }

        compiled_clusters.push(compiled_cluster);
    }

    Fragments {
        root,
        clusters: compiled_clusters,
        map: fragments,
        by_location: fragments_by_location,
    }
}

fn compile_function(
    function: syntax::Function,
    fragments: &mut FragmentMap,
    fragments_by_location: &mut FragmentsByLocation,
) -> Fragment {
    let mut branches = Vec::new();

    for branch in function.branches {
        let body = branch
            .body
            .into_iter()
            .map(|expression| {
                compile_expression(expression, fragments, fragments_by_location)
            })
            .collect::<Vec<_>>();
        let start = address_context(
            &body,
            &mut Vec::new(),
            fragments,
            fragments_by_location,
        );

        let body = iter::successors(Some(0), |i| Some(i + 1))
            .map(FragmentIndexInBranchBody)
            .zip(body)
            .collect();

        branches.push(Branch {
            parameters: Parameters {
                inner: branch.parameters,
            },
            start,
            body,
        });
    }

    let branches = iter::successors(Some(0), |i| Some(i + 1))
        .map(BranchIndex)
        .zip(branches)
        .collect();

    Fragment::Function {
        function: Function {
            name: function.name,
            branches,
            environment: function.environment,
            index_in_cluster: function.index_in_cluster,
        },
    }
}

fn address_context(
    context: &Vec<Fragment>,
    ids: &mut Vec<FragmentId>,
    fragments: &mut FragmentMap,
    fragments_by_location: &mut FragmentsByLocation,
) -> Option<FragmentId> {
    for fragment in context {
        ids.push(FragmentId {
            prev: None,
            next: None,
            content: Hash::new(fragment),
        });
    }

    let mut prev = None;

    for id in ids.iter_mut() {
        let prev_hash = prev.as_ref().map(Hash::new);

        id.prev = prev_hash;
        prev = Some(PrevNeighbor {
            ulterior_neighbor: prev_hash,
            content: id.content,
        });
    }

    let mut next = None;

    for id in ids.iter_mut().rev() {
        let next_hash = next.as_ref().map(Hash::new);

        id.next = next_hash;
        next = Some(NextNeighbor {
            ulterior_neighbor: next_hash,
            content: id.content,
        });
    }

    for (i, (fragment, id)) in context.iter().zip(&*ids).enumerate() {
        let previous = if i == 0 {
            None
        } else {
            ids.get(i - 1).copied()
        };
        let next = ids.get(i + 1).copied();

        fragments.insert(*id, fragment.clone(), previous, next);
        fragments_by_location.insert(*id, previous, next);
    }

    ids.first().copied()
}

fn compile_expression(
    expression: syntax::Expression,
    fragments: &mut FragmentMap,
    fragments_by_location: &mut FragmentsByLocation,
) -> Fragment {
    match expression {
        syntax::Expression::Comment { text } => Fragment::Comment { text },
        syntax::Expression::Function { function } => {
            compile_function(function, fragments, fragments_by_location)
        }
        syntax::Expression::Identifier {
            name,
            target,
            is_known_to_be_in_tail_position,
        } => {
            // By the time we make it to this compiler pass, all expressions
            // that are in tail position should be known to be so.
            let is_in_tail_position = is_known_to_be_in_tail_position;

            match target {
                Some(IdentifierTarget::Binding) => {
                    Fragment::ResolvedBinding { name }
                }
                Some(IdentifierTarget::Function {
                    is_known_to_be_recursive_call_to_index,
                }) => {
                    // By the time we make it to this compiler pass, all calls
                    // that are recursive should be known to be so.
                    let is_recursive_call_to_index =
                        is_known_to_be_recursive_call_to_index;

                    if let Some(index) = is_recursive_call_to_index {
                        Fragment::CallToFunctionRecursive {
                            index,
                            is_tail_call: is_in_tail_position,
                        }
                    } else {
                        Fragment::CallToFunction {
                            name,
                            is_tail_call: is_in_tail_position,
                        }
                    }
                }
                Some(IdentifierTarget::HostFunction { effect_number }) => {
                    Fragment::CallToHostFunction { effect_number }
                }
                Some(IdentifierTarget::Intrinsic { intrinsic }) => {
                    Fragment::CallToIntrinsic {
                        intrinsic,
                        is_tail_call: is_in_tail_position,
                    }
                }
                None => Fragment::UnresolvedIdentifier { name },
            }
        }
        syntax::Expression::Value(value) => Fragment::Value(value),
    }
}
