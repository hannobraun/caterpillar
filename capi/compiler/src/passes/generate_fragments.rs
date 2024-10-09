use std::{collections::BTreeMap, iter};

use crate::{
    fragments::{
        Branch, BranchIndex, Fragment, FragmentIndexInBranchBody, Fragments,
        Function, Parameters,
    },
    hash::Hash,
    syntax::{self, IdentifierTarget},
};

pub fn generate_fragments(clusters: syntax::Clusters) -> Fragments {
    let functions = clusters
        .clusters
        .iter()
        .rev()
        .flat_map(|cluster| cluster.functions.values())
        .map(|&index| {
            let mut functions = BTreeMap::new();

            let function = clusters.functions[&index].clone();
            let function = compile_function(function, &mut functions);

            let name = function.name.clone().expect(
                "Just compiled a named function; should have its name set.",
            );
            functions.insert(name, Hash::new(&function));

            (index, function)
        })
        .collect::<BTreeMap<_, _>>();

    Fragments {
        functions,
        clusters: clusters.clusters,
    }
}

fn compile_function(
    function: syntax::Function,
    functions: &mut BTreeMap<String, Hash<Function>>,
) -> Function {
    let mut branches = Vec::new();

    for branch in function.branches {
        let body = branch
            .body
            .into_iter()
            .map(|expression| compile_expression(expression, functions))
            .collect::<Vec<_>>();

        let body = iter::successors(Some(0), |i| Some(i + 1))
            .map(FragmentIndexInBranchBody)
            .zip(body)
            .collect();

        branches.push(Branch {
            parameters: Parameters {
                inner: branch.parameters,
            },
            body,
        });
    }

    let branches = iter::successors(Some(0), |i| Some(i + 1))
        .map(BranchIndex)
        .zip(branches)
        .collect();

    Function {
        name: function.name,
        branches,
        environment: function.environment,
        index_in_cluster: function.index_in_cluster,
    }
}

fn compile_expression(
    expression: syntax::Expression,
    functions: &mut BTreeMap<String, Hash<Function>>,
) -> Fragment {
    match expression {
        syntax::Expression::Comment { text } => Fragment::Comment { text },
        syntax::Expression::Function { function } => {
            let function = compile_function(function, functions);
            Fragment::Function { function }
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
