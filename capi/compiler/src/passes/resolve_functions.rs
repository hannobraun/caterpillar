use std::collections::BTreeMap;

use crate::code::{
    CallGraph, Expression, Function, FunctionLocation, Functions, Hash,
    StableFunctions,
};

pub fn resolve_calls_to_user_defined_functions(
    mut functions: Functions,
    call_graph: &CallGraph,
) -> StableFunctions {
    let mut resolved_hashes_by_name = BTreeMap::new();

    for (index, _) in call_graph.functions_from_leaves() {
        let FunctionLocation::NamedFunction { index } = index else {
            unreachable!("Only named functions are tracked in `CallGraph`");
        };

        let function = functions
            .named
            .get_mut(index)
            .expect("Function referred to from call graph must exist.");

        resolve_calls_in_function(
            &mut function.inner,
            &mut resolved_hashes_by_name,
        );

        resolved_hashes_by_name
            .insert(function.name.clone(), Hash::new(&function.inner));
    }

    // We have resolved the function calls, creating hashes to refer to the
    // resolved functions. From here on, the functions must not be allowed to
    // change, or those hashes will become invalid.
    StableFunctions::from(functions)
}

fn resolve_calls_in_function(
    function: &mut Function,
    resolved_hashes_by_name: &mut BTreeMap<String, Hash<Function>>,
) {
    for branch in function.branches.values_mut() {
        for expression in branch.body.values_mut() {
            resolve_calls_in_expression(expression, resolved_hashes_by_name);
        }
    }
}

fn resolve_calls_in_expression(
    expression: &mut Expression,
    resolved_hashes_by_name: &mut BTreeMap<String, Hash<Function>>,
) {
    match expression {
        Expression::LiteralFunction { function, .. } => {
            resolve_calls_in_function(function, resolved_hashes_by_name);
        }
        Expression::UnresolvedIdentifier {
            name,
            is_known_to_be_in_tail_position,
            is_known_to_be_call_to_user_defined_function,
        } => {
            // By the time we make it to this compiler pass, all expressions
            // that are in tail position should be known to be so.
            let is_in_tail_position = is_known_to_be_in_tail_position;

            if *is_known_to_be_call_to_user_defined_function {
                let Some(hash) = resolved_hashes_by_name.get(name).copied()
                else {
                    unreachable!(
                        "Resolving call to function `{name}`, but can't find \
                        the hash of that function. This should not happen, \
                        because this compiler pass processes functions by \
                        their order in the call graph.\n\
                        \n\
                        Theoretically, this could also happen if this is a \
                        recursive call. But those have been resolved in an \
                        earlier compiler pass already, and that shouldn't be a \
                        factor here."
                    );
                };

                *expression = Expression::CallToUserDefinedFunction {
                    hash,
                    is_tail_call: *is_in_tail_position,
                };
            }
        }
        _ => {}
    }
}
