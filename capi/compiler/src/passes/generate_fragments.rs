use std::{collections::BTreeMap, iter};

use crate::{
    code::{
        Branch, BranchIndex, CallGraph, Fragment, FragmentIndexInBranchBody,
        Function, FunctionIndexInRootContext, NamedFunctions,
    },
    hash::Hash,
};

pub fn generate_fragments(
    functions: BTreeMap<FunctionIndexInRootContext, Function>,
    call_graph: &CallGraph,
) -> NamedFunctions {
    let mut functions2 = BTreeMap::new();
    let mut named_functions = NamedFunctions::default();

    for (index, function) in functions {
        named_functions.insert(index, function);
    }

    for &index in call_graph.functions_from_leaves() {
        let function = named_functions
            .get(&index)
            .expect("Function referred to from call graph must exist.")
            .clone();
        let hash = Hash::new(&function);

        let function = compile_function(function, &mut functions2);

        functions2.insert(hash, Hash::new(&function));
    }

    named_functions
}

fn compile_function(
    function: Function,
    functions2: &mut BTreeMap<Hash<Function>, Hash<Function>>,
) -> Function {
    let mut branches = Vec::new();

    for branch in function.branches.into_values() {
        let body = branch
            .body
            .into_values()
            .map(|expression| compile_expression(expression, functions2))
            .collect::<Vec<_>>();

        let body = iter::successors(Some(0), |i| Some(i + 1))
            .map(FragmentIndexInBranchBody)
            .zip(body)
            .collect();

        branches.push(Branch {
            parameters: branch.parameters,
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
    expression: Fragment,
    functions2: &mut BTreeMap<Hash<Function>, Hash<Function>>,
) -> Fragment {
    match expression {
        Fragment::CallToHostFunction { effect_number } => {
            Fragment::CallToHostFunction { effect_number }
        }
        Fragment::CallToIntrinsicFunction {
            intrinsic,
            is_tail_call,
        } => Fragment::CallToIntrinsicFunction {
            intrinsic,
            is_tail_call,
        },
        Fragment::CallToUserDefinedFunction { hash, is_tail_call } => {
            let Some(hash) = functions2.get(&hash).copied() else {
                panic!(
                    "Compiling call to function `{hash:?}`. Expecting called \
                    function to already be compiled when its caller is being \
                    compiled."
                );
            };

            Fragment::CallToUserDefinedFunction { hash, is_tail_call }
        }
        Fragment::CallToUserDefinedFunctionRecursive {
            index,
            is_tail_call,
        } => Fragment::CallToUserDefinedFunctionRecursive {
            index,
            is_tail_call,
        },
        Fragment::Comment { text } => Fragment::Comment { text },
        Fragment::Function { function } => {
            let function = compile_function(function, functions2);
            Fragment::Function { function }
        }
        Fragment::ResolvedBinding { name } => {
            Fragment::ResolvedBinding { name }
        }
        Fragment::UnresolvedIdentifier {
            name,
            is_known_to_be_in_tail_position,
            is_known_to_be_call_to_user_defined_function,
        } => Fragment::UnresolvedIdentifier {
            name,
            is_known_to_be_in_tail_position,
            is_known_to_be_call_to_user_defined_function,
        },
        Fragment::Value(value) => Fragment::Value(value),
    }
}
