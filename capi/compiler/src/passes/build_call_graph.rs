use std::collections::BTreeMap;

use petgraph::{
    algo::{condensation, toposort},
    Graph,
};

use crate::code::{
    CallGraph, Cluster, Expression, FunctionLocation, Functions, IndexMap,
};

pub fn build_call_graph(functions: &Functions) -> CallGraph {
    let dependency_graph = build_dependency_graph(functions);
    let clusters = collect_functions_into_clusters(dependency_graph);
    CallGraph::from_clusters(clusters)
}

fn build_dependency_graph(functions: &Functions) -> DependencyGraph {
    let mut call_graph = Graph::new();
    let mut graph_index_by_function_location = BTreeMap::new();

    for function in functions.all_functions() {
        graph_index_by_function_location
            .entry(function.location.clone())
            .or_insert_with(|| call_graph.add_node(function.location));
    }

    for function in functions.all_functions() {
        let depender = graph_index_by_function_location[&function.location];

        for branch in function.branches() {
            for expression in branch.body() {
                let dependee = match expression.fragment {
                    Expression::UnresolvedIdentifier {
                        name,
                        is_known_to_be_call_to_user_defined_function: true,
                        ..
                    } => {
                        let named_function =
                            functions.named.by_name(name).expect(
                                "Just got `name` from an identifier that is \
                                known to be a call to a user-defined function. \
                                A function of that name must be available.",
                            );
                        Some(named_function.location())
                    }
                    _ => expression.to_function_location(),
                };

                if let Some(dependee) = dependee {
                    let dependee = graph_index_by_function_location[&dependee];
                    call_graph.add_edge(depender, dependee, ());
                }
            }
        }
    }

    call_graph
}

fn collect_functions_into_clusters(
    dependency_graph: DependencyGraph,
) -> impl Iterator<Item = Cluster> {
    let make_acyclic = true;
    let mut clustered_graph = condensation(dependency_graph, make_acyclic);

    let clustered_and_sorted_graph = toposort(&clustered_graph, None).expect(
        "The previous operation should have made the call graph acyclic. \
        Hence, topologically sorting the graph should not fail.",
    );

    clustered_and_sorted_graph
        .into_iter()
        .filter_map(move |graph_index| {
            let function_group =
                clustered_graph.remove_node(graph_index).expect(
                    "Each entry in the sorted version of the call graph must \
                    correspond to exactly one node in the unsorted version. So \
                    using every node from the sorted version once, to remove
                    its respective node in the unsorted version, should always \
                    work.",
                );

            let mut named_functions = IndexMap::default();
            for location in function_group {
                let FunctionLocation::NamedFunction { index } = location else {
                    continue;
                };
                named_functions.push(index);
            }

            if named_functions.is_empty() {
                None
            } else {
                Some(Cluster {
                    functions: named_functions,
                    divergent_functions: None,
                    non_divergent_branches: None,
                })
            }
        })
}

type DependencyGraph = Graph<FunctionLocation, ()>;

#[cfg(test)]
mod tests {
    use crate::{
        code::{CallGraph, Cluster, Index},
        host::NoHost,
        passes::{parse, resolve_most_identifiers, tokenize},
    };

    #[test]
    fn no_recursion() {
        let call_graph = create_call_graph(
            r"
                main: fn
                    \ ->
                        f
                end

                f: fn
                    \ ->
                        nop
                end
            ",
        );

        assert_eq!(
            call_graph
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [
                (Index::from(0), Index::from(1)),
                (Index::from(0), Index::from(0)),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: [indices].into_iter().collect(),
                divergent_functions: None,
                non_divergent_branches: None,
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn self_recursion() {
        let call_graph = create_call_graph(
            r"
                main: fn
                    \ ->
                        f
                end

                f: fn
                    \ ->
                        f
                end
            ",
        );

        assert_eq!(
            call_graph
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [
                (Index::from(0), Index::from(1)),
                (Index::from(0), Index::from(0)),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: [indices].into_iter().collect(),
                divergent_functions: None,
                non_divergent_branches: None,
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn mutual_recursion() {
        let call_graph = create_call_graph(
            r"
                main: fn
                    \ ->
                        f
                end

                f: fn
                    \ ->
                        g
                end

                g: fn
                    \ ->
                        f
                end
            ",
        );

        assert_eq!(
            call_graph
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [
                [
                    (Index::from(0), Index::from(1)),
                    (Index::from(1), Index::from(2))
                ]
                .as_slice(),
                [(Index::from(0), Index::from(0))].as_slice(),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: indices.iter().copied().collect(),
                divergent_functions: None,
                non_divergent_branches: None,
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn sort_clusters_by_call_graph() {
        let call_graph = create_call_graph(
            r"
                main: fn
                    \ ->
                        a
                end

                a: fn
                    \ ->
                        # Call a function that comes is placed after this one in
                        # the source code.
                        c
                end

                b: fn
                    \ ->
                        nop
                end

                c: fn
                    \ ->
                        # And for some variety, call a function that is placed
                        # before.
                        b
                end
            ",
        );

        assert_eq!(
            call_graph
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [
                [(Index::from(0), Index::from(2))].as_slice(),
                [(Index::from(0), Index::from(3))].as_slice(),
                [(Index::from(0), Index::from(1))].as_slice(),
                [(Index::from(0), Index::from(0))].as_slice(),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: indices.iter().copied().collect(),
                divergent_functions: None,
                non_divergent_branches: None,
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn consider_anonymous_functions_in_call_graph() {
        let call_graph = create_call_graph(
            r"
                a: fn
                    \ ->
                        fn
                            \ ->
                                # Call a function that is placed after this one
                                # in the source code.
                                c
                        end
                end

                b: fn
                    \ ->
                        nop
                end

                c: fn
                    \ ->
                        fn
                            \ ->
                                # And for some variety, call a function that is
                                # placed before.
                                b
                        end
                end
            ",
        );

        assert_eq!(
            call_graph
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [
                [(Index::from(0), Index::from(1))].as_slice(),
                [(Index::from(0), Index::from(2))].as_slice(),
                [(Index::from(0), Index::from(0))].as_slice(),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: indices.iter().copied().collect(),
                divergent_functions: None,
                non_divergent_branches: None,
            })
            .collect::<Vec<_>>(),
        );
    }

    fn create_call_graph(source: &str) -> CallGraph {
        let tokens = tokenize(source);
        let mut functions = parse(tokens);
        resolve_most_identifiers(&mut functions, &NoHost);
        super::build_call_graph(&functions)
    }
}
