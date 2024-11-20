use std::collections::BTreeMap;

use crate::code::{
    Expression, ExpressionLocation, Function, FunctionLocation, Functions,
    Hash, Located, OrderedFunctions, StableFunctions, TailExpressions,
};

pub fn resolve_non_recursive_functions(
    mut functions: Functions,
    call_graph: &OrderedFunctions,
    tail_expressions: &TailExpressions,
) -> StableFunctions {
    let mut resolved_hashes_by_name = BTreeMap::new();
    let mut resolved_hashes_by_location = BTreeMap::new();

    for (location, _) in call_graph.functions_from_leaves() {
        let function = functions
            .by_location_mut(location)
            .expect("Function referred to from call graph must exist.");

        if let Err(err) = resolve_calls_in_function(
            function,
            tail_expressions,
            &resolved_hashes_by_name,
            &resolved_hashes_by_location,
        ) {
            unreachable!("Unexpected error: {err:?}");
        }

        match location {
            FunctionLocation::NamedFunction { index } => {
                let function = functions.named.by_index(index).expect(
                    "Got function index by iterating over ordered functions. \
                    Function referred to from there must exist.",
                );
                resolved_hashes_by_name
                    .insert(function.name.clone(), Hash::new(&function.inner));
            }
            FunctionLocation::AnonymousFunction { location } => {
                let function =
                    functions.anonymous.by_location(location).expect(
                        "Got function location by iterating over ordered \
                        functions. Function referred to from there must exist.",
                    );
                resolved_hashes_by_location
                    .insert(location.clone(), Hash::new(function.fragment));
            }
        }
    }

    // We have resolved the function calls, creating hashes to refer to the
    // resolved functions. From here on, the functions must not be allowed to
    // change, or those hashes will become invalid.
    StableFunctions::from(functions)
}

fn resolve_calls_in_function(
    function: Located<&mut Function>,
    tail_expressions: &TailExpressions,
    resolved_hashes_by_name: &BTreeMap<String, Hash<Function>>,
    resolved_hashes_by_location: &BTreeMap<ExpressionLocation, Hash<Function>>,
) -> Result<(), ExpressionLocation> {
    let (branches, _) = function.destructure();

    for branch in branches {
        let (body, _) = branch.destructure();

        for expression in body {
            resolve_calls_in_expression(
                expression,
                tail_expressions,
                resolved_hashes_by_name,
                resolved_hashes_by_location,
            )?;
        }
    }

    Ok(())
}

fn resolve_calls_in_expression(
    expression: Located<&mut Expression>,
    tail_expressions: &TailExpressions,
    resolved_hashes_by_name: &BTreeMap<String, Hash<Function>>,
    resolved_hashes_by_location: &BTreeMap<ExpressionLocation, Hash<Function>>,
) -> Result<(), ExpressionLocation> {
    match expression.fragment {
        Expression::UnresolvedIdentifier {
            name,
            is_known_to_be_call_to_user_defined_function,
        } => {
            if *is_known_to_be_call_to_user_defined_function {
                let is_tail_call =
                    tail_expressions.is_tail_expression(&expression.location);

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

                *expression.fragment = Expression::CallToUserDefinedFunction {
                    hash,
                    is_tail_call,
                };
            }
        }
        Expression::UnresolvedLocalFunction => {
            let hash = *resolved_hashes_by_location
                .get(&expression.location)
                .expect(
                    "This compiler pass iterates over functions by \
                    dependency. Any local function that is encountered should \
                    have already been processed. We should have a hash for it.",
                );
            *expression.fragment = Expression::LocalFunction { hash };
        }
        _ => {}
    }

    Ok(())
}