use std::collections::BTreeMap;

use petgraph::{
    algo::{condensation, toposort},
    Graph,
};

use crate::code::{
    CallGraph, Cluster, Expression, FunctionLocation, Functions, IndexMap,
};

pub fn build_call_graph(functions: &Functions) -> CallGraph {
    let pet_call_graph = build_pet_call_graph(functions);
    let clusters = collect_functions_into_clusters(pet_call_graph, functions);
    CallGraph::from_clusters(clusters)
}

fn build_pet_call_graph(functions: &Functions) -> PetCallGraph {
    let mut call_graph = Graph::new();
    let mut graph_index_by_function_location = BTreeMap::new();

    for named_function in functions.all_functions() {
        let location = named_function.location;

        graph_index_by_function_location
            .entry(location.clone())
            .or_insert_with(|| call_graph.add_node(location));
    }

    for function in functions.all_functions() {
        let self_index =
            {
                let named_function =
                functions.named.by_child_function(&function.location).expect(
                    "All functions are either named, or defined in a named \
                    function. Just got `function` from `functions`. If it \
                    isn't a named function already, then the named function \
                    it's defined in must be in there.",
                );
                graph_index_by_function_location[&named_function.location()]
            };

        for branch in function.branches() {
            for expression in branch.body() {
                if let Expression::UnresolvedIdentifier {
                    name,
                    is_known_to_be_call_to_user_defined_function: true,
                    ..
                } = expression.fragment
                {
                    let named_function = functions.named.by_name(name).expect(
                        "Just got `name` from an identifier that is known to \
                        be a call to a user-defined function. A function of \
                        that name must be available.",
                    );
                    let callee_index = graph_index_by_function_location
                        [&named_function.location()];
                    call_graph.add_edge(self_index, callee_index, ());
                }
            }
        }
    }

    call_graph
}

fn collect_functions_into_clusters(
    call_graph: PetCallGraph,
    _: &Functions,
) -> impl Iterator<Item = Cluster> {
    let make_acyclic = true;
    let mut clustered_call_graph = condensation(call_graph, make_acyclic);

    let clustered_and_sorted_call_graph = toposort(&clustered_call_graph, None)
        .expect(
            "The previous operation should have made the call graph acyclic. \
            Hence, topologically sorting the graph should not fail.",
        );

    clustered_and_sorted_call_graph
        .into_iter()
        .filter_map(move |graph_index| {
            let function_group =
                clustered_call_graph.remove_node(graph_index).expect(
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

type PetCallGraph = Graph<FunctionLocation, ()>;

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

    fn create_call_graph(source: &str) -> CallGraph {
        let tokens = tokenize(source);
        let mut functions = parse(tokens);
        resolve_most_identifiers(&mut functions, &NoHost);
        super::build_call_graph(&functions)
    }
}
