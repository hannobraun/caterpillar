use std::{collections::BTreeMap, iter};

use crate::{
    fragments::{
        Branch, BranchIndex, CallGraph, Fragment, FragmentIndexInBranchBody,
        Function, FunctionIndexInRootContext, NamedFunctions, Parameters,
    },
    hash::Hash,
    syntax,
};

pub fn generate_fragments(
    functions: BTreeMap<FunctionIndexInRootContext, syntax::Function>,
    call_graph: &CallGraph,
) -> NamedFunctions {
    let mut hashes = BTreeMap::new();
    let mut functions2 = BTreeMap::new();
    let mut named_functions = NamedFunctions::default();

    for &index in call_graph.functions_from_leaves() {
        let function = functions[&index].clone();
        let hash = Hash::new(&function);

        let function = compile_function(function, &mut hashes, &mut functions2);

        let name = function.name.clone().expect(
            "Just compiled a named function; should have its name set.",
        );
        hashes.insert(name, Hash::new(&function));
        functions2.insert(hash, Hash::new(&function));

        named_functions.insert(index, function);
    }

    named_functions
}

fn compile_function(
    function: syntax::Function,
    functions: &mut BTreeMap<String, Hash<Function>>,
    functions2: &mut BTreeMap<Hash<syntax::Function>, Hash<Function>>,
) -> Function {
    let mut branches = Vec::new();

    for branch in function.branches {
        let body = branch
            .body
            .into_iter()
            .map(|expression| {
                compile_expression(expression, functions, functions2)
            })
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
    functions2: &mut BTreeMap<Hash<syntax::Function>, Hash<Function>>,
) -> Fragment {
    match expression {
        syntax::Expression::CallToHostFunction { effect_number } => {
            Fragment::CallToHostFunction { effect_number }
        }
        syntax::Expression::CallToIntrinsicFunction {
            intrinsic,
            is_tail_call,
        } => Fragment::CallToIntrinsicFunction {
            intrinsic,
            is_tail_call,
        },
        syntax::Expression::CallToUserDefinedFunction {
            hash,
            is_tail_call,
        } => {
            let Some(hash) = functions2.get(&hash).copied() else {
                panic!(
                    "Compiling call to function `{hash:?}`. Expecting called \
                    function to already be compiled when its caller is being \
                    compiled."
                );
            };

            Fragment::CallToUserDefinedFunction { hash, is_tail_call }
        }
        syntax::Expression::CallToUserDefinedFunctionRecursive {
            index,
            is_tail_call,
        } => Fragment::CallToUserDefinedFunctionRecursive {
            index,
            is_tail_call,
        },
        syntax::Expression::Comment { text } => Fragment::Comment { text },
        syntax::Expression::Function { function } => {
            let function = compile_function(function, functions, functions2);
            Fragment::Function { function }
        }
        syntax::Expression::ResolvedBinding { name } => {
            Fragment::ResolvedBinding { name }
        }
        syntax::Expression::UnresolvedIdentifier {
            name,
            is_known_to_be_in_tail_position,
            is_known_to_be_call_to_user_defined_function: _,
        } => Fragment::UnresolvedIdentifier {
            name,
            is_known_to_be_in_tail_position,
        },
        syntax::Expression::Value(value) => Fragment::Value(value),
    }
}
