use std::{collections::BTreeMap, iter};

use petgraph::{
    algo::{condensation, toposort},
    graph::NodeIndex,
    Graph,
};

use crate::{
    fragments::{FunctionIndexInCluster, FunctionIndexInRootContext},
    syntax::{Cluster, Clusters, Expression, Function, IdentifierTarget},
};

pub fn group_into_clusters(functions: Vec<Function>) -> Clusters {
    let functions = iter::successors(Some(0), |i| Some(i + 1))
        .map(FunctionIndexInRootContext)
        .zip(functions)
        .collect::<BTreeMap<_, _>>();

    let mut call_graph = Graph::new();
    let mut graph_index_by_function_name = BTreeMap::new();

    for (named_function_index, function) in functions.iter() {
        let name = function
            .name
            .as_ref()
            .expect("Top-level function must have a name");
        let graph_index =
            call_graph.add_node((function, *named_function_index));
        graph_index_by_function_name.insert(name, graph_index);
    }

    for &caller_index in graph_index_by_function_name.values() {
        let (function, _) = call_graph[caller_index];

        include_calls_from_function_in_call_graph(
            caller_index,
            function,
            &graph_index_by_function_name,
            &mut call_graph,
        );
    }

    let make_acyclic = true;
    let clustered_call_graph = condensation(call_graph, make_acyclic);
    let clustered_and_sorted_call_graph = toposort(&clustered_call_graph, None)
        .expect(
            "The previous operation should have made the call graph acyclic. \
            Hence, topologically sorting the graph should not fail.",
        );
    let clusters = clustered_and_sorted_call_graph
        .into_iter()
        .map(|graph_index| {
            let named_function_indices = clustered_call_graph[graph_index]
                .iter()
                .map(|(_, named_function_index)| named_function_index)
                .copied();
            let functions = iter::successors(Some(0), |i| Some(i + 1))
                .map(FunctionIndexInCluster)
                .zip(named_function_indices)
                .collect();

            Cluster { functions }
        })
        .collect();

    Clusters {
        functions,
        clusters,
    }
}

fn include_calls_from_function_in_call_graph(
    caller_index: NodeIndex,
    function: &Function,
    graph_index_by_function_name: &BTreeMap<&String, NodeIndex>,
    call_graph: &mut Graph<(&Function, FunctionIndexInRootContext), ()>,
) {
    for branch in &function.branches {
        for expression in &branch.body {
            if let Expression::Identifier {
                name,
                target: Some(IdentifierTarget::Function { .. }),
                ..
            } = expression
            {
                let callee_index = graph_index_by_function_name[name];
                call_graph.add_edge(caller_index, callee_index, ());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::{
        fragments::{FunctionIndexInCluster, FunctionIndexInRootContext},
        host::NoHost,
        passes::{parse, resolve_identifiers, tokenize},
        syntax::{Cluster, Clusters},
    };

    #[test]
    fn no_recursion() {
        let clusters = group_into_clusters(
            r"
                main: {
                    \ ->
                        f
                }

                f: {
                    \ ->
                        nop
                }
            ",
        );

        assert_eq!(
            clusters.clusters,
            [
                (FunctionIndexInCluster(0), FunctionIndexInRootContext(0)),
                (FunctionIndexInCluster(0), FunctionIndexInRootContext(1)),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: BTreeMap::from([indices]),
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn self_recursion() {
        let clusters = group_into_clusters(
            r"
                main: {
                    \ ->
                        f
                }

                f: {
                    \ ->
                        f
                }
            ",
        );

        assert_eq!(
            clusters.clusters,
            [
                (FunctionIndexInCluster(0), FunctionIndexInRootContext(0)),
                (FunctionIndexInCluster(0), FunctionIndexInRootContext(1))
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: BTreeMap::from([indices])
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn mutual_recursion() {
        let clusters = group_into_clusters(
            r"
                main: {
                    \ ->
                        f
                }

                f: {
                    \ ->
                        g
                }

                g: {
                    \ ->
                        f
                }
            ",
        );

        assert_eq!(
            clusters.clusters,
            [
                [(FunctionIndexInCluster(0), FunctionIndexInRootContext(0))]
                    .as_slice(),
                [
                    (FunctionIndexInCluster(0), FunctionIndexInRootContext(1)),
                    (FunctionIndexInCluster(1), FunctionIndexInRootContext(2))
                ]
                .as_slice(),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: indices.iter().copied().collect(),
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn sort_clusters_by_call_graph() {
        let clusters = group_into_clusters(
            r"
                main: {
                    \ ->
                        a
                }

                a: {
                    \ ->
                        # Call a function that comes is placed after this one in
                        # the source code.
                        c
                }

                b: {
                    \ ->
                        nop
                }

                c: {
                    \ ->
                        # And for some variety, call a function that is placed
                        # before.
                        b
                }
            ",
        );

        assert_eq!(
            clusters.clusters,
            [
                [(FunctionIndexInCluster(0), FunctionIndexInRootContext(0))]
                    .as_slice(),
                [(FunctionIndexInCluster(0), FunctionIndexInRootContext(1))]
                    .as_slice(),
                [(FunctionIndexInCluster(0), FunctionIndexInRootContext(3))]
                    .as_slice(),
                [(FunctionIndexInCluster(0), FunctionIndexInRootContext(2))]
                    .as_slice(),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: indices.iter().copied().collect(),
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    #[should_panic] // known bug; currently not tracked in an issue
    fn consider_anonymous_functions_in_call_graph() {
        let clusters = group_into_clusters(
            r"
                main: {
                    \ ->
                        {
                            \ ->
                                a
                        }
                }

                a: {
                    \ ->
                        {
                            \ ->
                                # Call a function that comes is placed after
                                # this one in the source code.
                                c
                        }
                }

                b: {
                    \ ->
                        nop
                }

                c: {
                    \ ->
                        {
                            \ ->
                                # And for some variety, call a function that is
                                # placed before.
                                b
                        }
                }
            ",
        );

        assert_eq!(
            clusters.clusters,
            [
                [(FunctionIndexInCluster(0), FunctionIndexInRootContext(0))]
                    .as_slice(),
                [(FunctionIndexInCluster(0), FunctionIndexInRootContext(1))]
                    .as_slice(),
                [(FunctionIndexInCluster(0), FunctionIndexInRootContext(3))]
                    .as_slice(),
                [(FunctionIndexInCluster(0), FunctionIndexInRootContext(2))]
                    .as_slice(),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: indices.iter().copied().collect(),
            })
            .collect::<Vec<_>>(),
        );
    }

    fn group_into_clusters(source: &str) -> Clusters {
        let tokens = tokenize(source);
        let mut functions = parse(tokens);
        resolve_identifiers::<NoHost>(&mut functions);
        super::group_into_clusters(functions)
    }
}
