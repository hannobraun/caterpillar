use std::collections::BTreeMap;

use crate::code::{
    CallGraph, Fragment, Function, FunctionIndexInCluster, NamedFunctions,
    UnresolvedCallToUserDefinedFunction,
};

pub fn mark_recursive_calls(
    functions: &mut NamedFunctions,
    call_graph: &CallGraph,
) {
    for cluster in call_graph.clusters_from_leaves() {
        let indices_in_cluster_by_function_name = cluster
            .functions
            .iter()
            .filter_map(|(&function_index_in_cluster, named_function_index)| {
                functions
                    .get(named_function_index)
                    .expect(
                        "Expecting function referenced from call graph to \
                        exist.",
                    )
                    .name
                    .clone()
                    .map(|name| (name, function_index_in_cluster))
            })
            .collect::<BTreeMap<_, _>>();

        for named_function_index in cluster.functions.values() {
            let function = functions
                .get_mut(named_function_index)
                .expect("Functions referred to from clusters must exist.");

            mark_recursive_calls_in_function(
                function,
                &indices_in_cluster_by_function_name,
            );
        }
    }
}

fn mark_recursive_calls_in_function(
    function: &mut Function,
    indices_in_cluster_by_function_name: &BTreeMap<
        String,
        FunctionIndexInCluster,
    >,
) {
    for branch in function.branches.values_mut() {
        for expression in branch.body.values_mut() {
            match expression {
                Fragment::Function { function } => {
                    mark_recursive_calls_in_function(
                        function,
                        indices_in_cluster_by_function_name,
                    );
                }
                Fragment::UnresolvedIdentifier {
                    name,
                    is_known_to_be_call_to_user_defined_function:
                        Some(UnresolvedCallToUserDefinedFunction {
                            is_known_to_be_recursive_call,
                        }),
                    ..
                } => {
                    if let Some(&index) =
                        indices_in_cluster_by_function_name.get(name)
                    {
                        *is_known_to_be_recursive_call = Some(index);
                    }
                }
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        code::{Fragment, NamedFunctions, UnresolvedCallToUserDefinedFunction},
        host::NoHost,
        passes::{
            create_call_graph, parse, resolve_most_identifiers, tokenize,
        },
    };

    #[test]
    fn self_recursive_functions() {
        let functions = mark_recursive_calls(
            r"
                f: {
                    \ ->
                        f
                }

                # Need a second function in this test, because for the first,
                # the index of the function within the list of all named
                # functions and the index of the function within its cluster,
                # are both `0`.
                #
                # This is prone to hide a bug, so we have this second function,
                # that has a non-zero index in the list of all named functions.
                g: {
                    \ ->
                        g
                }
            ",
        );

        for mut function in functions.into_functions() {
            let Fragment::UnresolvedIdentifier {
                is_known_to_be_call_to_user_defined_function,
                ..
            } = function
                .branches
                .pop_first()
                .map(|(_, branch)| branch)
                .unwrap()
                .body
                .pop_first()
                .map(|(_, fragment)| fragment)
                .unwrap()
            else {
                panic!("Expected expression to be an identifier.");
            };
            let Some(UnresolvedCallToUserDefinedFunction {
                is_known_to_be_recursive_call,
            }) = is_known_to_be_call_to_user_defined_function
            else {
                panic!("Expected expression to be a function call");
            };
            let Some(index) = is_known_to_be_recursive_call else {
                panic!("Expected function call to be recursive.");
            };

            assert_eq!(
                index.0, 0,
                "Function is only self-recursive, not mutually recursive. \
                Expecting it to be alone in a cluster, hence index referring \
                to it must be `0`."
            );
        }
    }

    #[test]
    fn mark_recursive_calls_from_anonymous_functions() {
        let functions = mark_recursive_calls(
            r"
                f: {
                    \ ->
                        {
                            \ ->
                                f
                        }
                }
            ",
        );

        let mut functions = functions.into_functions();
        let f = functions.next().unwrap();
        assert!(functions.next().is_none());

        let mut branches = f.branches.into_values();
        let branch = branches.next().unwrap();
        assert!(branches.next().is_none());

        let mut body = branch.body.into_values();
        let expression = body.next().unwrap();
        assert!(body.next().is_none());

        let Fragment::Function { function } = expression else {
            panic!("Expected expression to be a function.");
        };

        let mut branches = function.branches.into_values();
        let branch = branches.next().unwrap();
        assert!(branches.next().is_none());

        let mut body = branch.body.into_values();
        let expression = body.next().unwrap();
        assert!(body.next().is_none());

        let Fragment::UnresolvedIdentifier {
            is_known_to_be_call_to_user_defined_function:
                Some(UnresolvedCallToUserDefinedFunction {
                    is_known_to_be_recursive_call: Some(_),
                }),
            ..
        } = expression
        else {
            panic!("Expected identifier to be a recursive function call.");
        };
    }

    fn mark_recursive_calls(source: &str) -> NamedFunctions {
        let tokens = tokenize(source);
        let mut named_functions = parse(tokens);
        resolve_most_identifiers::<NoHost>(&mut named_functions);
        let call_graph = create_call_graph(&named_functions);
        super::mark_recursive_calls(&mut named_functions, &call_graph);

        named_functions
    }
}
