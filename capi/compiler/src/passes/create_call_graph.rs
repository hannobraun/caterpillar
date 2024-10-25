use std::collections::BTreeMap;

use petgraph::{
    algo::{condensation, toposort},
    graph::NodeIndex,
    Graph,
};

use crate::code::{
    CallGraph, Cluster, Fragment, Function, Index, IndexMap, NamedFunctions,
};

pub fn create_call_graph(named_functions: &NamedFunctions) -> CallGraph {
    let mut call_graph = Graph::new();
    let mut graph_index_by_function_name = BTreeMap::new();

    for (named_function_index, function) in named_functions {
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
    let clusters =
        clustered_and_sorted_call_graph
            .into_iter()
            .map(|graph_index| {
                let named_function_indices = clustered_call_graph[graph_index]
                    .iter()
                    .map(|(_, named_function_index)| named_function_index)
                    .copied();

                let mut functions = IndexMap::default();
                for index in named_function_indices {
                    functions.push(index);
                }

                Cluster { functions }
            });

    clusters.collect()
}

fn include_calls_from_function_in_call_graph(
    caller_index: NodeIndex,
    function: &Function,
    graph_index_by_function_name: &BTreeMap<&String, NodeIndex>,
    call_graph: &mut Graph<(&Function, Index<Function>), ()>,
) {
    for branch in function.branches.values() {
        for typed_fragment in branch.body.values() {
            match &typed_fragment.fragment {
                Fragment::Function { function } => {
                    include_calls_from_function_in_call_graph(
                        caller_index,
                        function,
                        graph_index_by_function_name,
                        call_graph,
                    );
                }
                Fragment::UnresolvedIdentifier {
                    name,
                    is_known_to_be_call_to_user_defined_function: Some(_),
                    ..
                } => {
                    let callee_index = graph_index_by_function_name[name];
                    call_graph.add_edge(caller_index, callee_index, ());
                }
                _ => {}
            }
        }
    }
}

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
                functions: [indices].into_iter().collect()
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
                functions: indices.iter().copied().collect()
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn consider_anonymous_functions_in_call_graph() {
        let call_graph = create_call_graph(
            r"
                main: fn
                    \ ->
                        fn
                            \ ->
                                a
                        end
                end

                a: fn
                    \ ->
                        fn
                            \ ->
                                # Call a function that comes is placed after
                                # this one in the source code.
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
                [(Index::from(0), Index::from(2))].as_slice(),
                [(Index::from(0), Index::from(3))].as_slice(),
                [(Index::from(0), Index::from(1))].as_slice(),
                [(Index::from(0), Index::from(0))].as_slice(),
            ]
            .into_iter()
            .map(|indices| Cluster {
                functions: indices.iter().copied().collect()
            })
            .collect::<Vec<_>>(),
        );
    }

    fn create_call_graph(source: &str) -> CallGraph {
        let tokens = tokenize(source);
        let mut named_functions = parse(tokens);
        resolve_most_identifiers::<NoHost>(&mut named_functions);
        super::create_call_graph(&named_functions)
    }
}
