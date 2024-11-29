use std::collections::BTreeMap;

use petgraph::{
    algo::{condensation, toposort},
    Graph,
};

use crate::code::{
    Cluster, Expression, FunctionCalls, FunctionLocation, Functions, IndexMap,
    OrderedFunctions,
};

pub fn order_functions_by_dependencies(
    functions: &Functions,
    function_calls: &FunctionCalls,
) -> OrderedFunctions {
    let dependency_graph = build_dependency_graph(functions, function_calls);
    let clusters = collect_functions_into_clusters(dependency_graph);
    OrderedFunctions::from_clusters(clusters)
}

fn build_dependency_graph(
    functions: &Functions,
    function_calls: &FunctionCalls,
) -> DependencyGraph {
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
                    Expression::Identifier { .. } => function_calls
                        .is_call_to_user_defined_function(&expression.location)
                        .cloned(),
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
                named_functions.push(location);
            }

            if named_functions.is_empty() {
                None
            } else {
                Some(Cluster {
                    functions: named_functions,
                })
            }
        })
}

type DependencyGraph = Graph<FunctionLocation, ()>;

#[cfg(test)]
mod tests {
    use crate::{
        code::{
            syntax::parse, Cluster, FunctionCalls, Functions, Index,
            OrderedFunctions, Tokens,
        },
        host::NoHost,
    };

    #[test]
    fn no_recursion() {
        let (functions, ordered_functions) = order_functions_by_dependencies(
            r"
                f: fn
                    \ ->
                        g
                end

                g: fn
                    \ ->
                        nop
                end
            ",
        );

        let f = functions.named.by_name("f").unwrap().location();
        let g = functions.named.by_name("g").unwrap().location();

        assert_eq!(
            ordered_functions
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [(Index::from(0), g), (Index::from(0), f),]
                .into_iter()
                .map(|locations_by_index| Cluster {
                    functions: [locations_by_index].into_iter().collect(),
                })
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn self_recursion() {
        let (functions, ordered_functions) = order_functions_by_dependencies(
            r"
                f: fn
                    \ ->
                        g
                end

                g: fn
                    \ ->
                        g
                end
            ",
        );

        let f = functions.named.by_name("f").unwrap().location();
        let g = functions.named.by_name("g").unwrap().location();

        assert_eq!(
            ordered_functions
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [(Index::from(0), g), (Index::from(0), f),]
                .into_iter()
                .map(|locations_by_index| Cluster {
                    functions: [locations_by_index].into_iter().collect(),
                })
                .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn mutual_recursion() {
        let (functions, ordered_functions) = order_functions_by_dependencies(
            r"
                f: fn
                    \ ->
                        g
                end

                g: fn
                    \ ->
                        h
                end

                h: fn
                    \ ->
                        g
                end
            ",
        );

        let f = functions.named.by_name("f").unwrap().location();
        let g = functions.named.by_name("g").unwrap().location();
        let h = functions.named.by_name("h").unwrap().location();

        assert_eq!(
            ordered_functions
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [
                [(Index::from(0), g), (Index::from(1), h)].as_slice(),
                [(Index::from(0), f)].as_slice(),
            ]
            .into_iter()
            .map(|locations_by_index| Cluster {
                functions: locations_by_index.iter().cloned().collect(),
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn sort_clusters_by_call_graph() {
        let (functions, ordered_functions) = order_functions_by_dependencies(
            r"
                f: fn
                    \ ->
                        # Call a function that comes is placed after this one in
                        # the source code.
                        h
                end

                g: fn
                    \ ->
                        nop
                end

                h: fn
                    \ ->
                        # And for some variety, call a function that is placed
                        # before.
                        g
                end
            ",
        );

        let f = functions.named.by_name("f").unwrap().location();
        let g = functions.named.by_name("g").unwrap().location();
        let h = functions.named.by_name("h").unwrap().location();

        assert_eq!(
            ordered_functions
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [
                [(Index::from(0), g)].as_slice(),
                [(Index::from(0), h)].as_slice(),
                [(Index::from(0), f)].as_slice(),
            ]
            .into_iter()
            .map(|locations_by_index| Cluster {
                functions: locations_by_index.iter().cloned().collect(),
            })
            .collect::<Vec<_>>(),
        );
    }

    #[test]
    fn consider_anonymous_functions_in_call_graph() {
        let (functions, ordered_functions) = order_functions_by_dependencies(
            r"
                f: fn
                    \ ->
                        fn
                            \ ->
                                # Call a function that is placed after this one
                                # in the source code.
                                h
                        end
                end

                g: fn
                    \ ->
                        nop
                end

                h: fn
                    \ ->
                        fn
                            \ ->
                                # And for some variety, call a function that is
                                # placed before.
                                g
                        end
                end
            ",
        );

        let [(f, f_a), (h, h_a)] = ["f", "h"].map(|name| {
            let named = functions.named.by_name(name).unwrap();
            let anonymous = named
                .into_located_function()
                .find_single_branch()
                .unwrap()
                .body()
                .next()
                .unwrap()
                .to_function_location()
                .unwrap();

            (named.location(), anonymous)
        });
        let g = functions.named.by_name("g").unwrap().location();

        assert_eq!(
            ordered_functions
                .clusters_from_leaves()
                .cloned()
                .collect::<Vec<_>>(),
            [
                [(Index::from(0), g)].as_slice(),
                [(Index::from(0), h_a)].as_slice(),
                [(Index::from(0), h)].as_slice(),
                [(Index::from(0), f_a)].as_slice(),
                [(Index::from(0), f)].as_slice(),
            ]
            .into_iter()
            .map(|locations_by_index| Cluster {
                functions: locations_by_index.iter().cloned().collect(),
            })
            .collect::<Vec<_>>(),
        );
    }

    fn order_functions_by_dependencies(
        input: &str,
    ) -> (Functions, OrderedFunctions) {
        let tokens = Tokens::tokenize(input);
        let (_syntax_tree, functions) = parse(tokens);
        let function_calls = FunctionCalls::resolve(&functions, &NoHost);
        let ordered_functions =
            super::order_functions_by_dependencies(&functions, &function_calls);
        (functions, ordered_functions)
    }
}
