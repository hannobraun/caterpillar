use std::collections::BTreeMap;

use petgraph::{
    algo::{condensation, toposort},
    Graph,
};

use crate::code::{
    syntax::{Expression, FunctionLocation, SyntaxTree},
    Cluster, FunctionCalls, IndexMap, OrderedFunctions,
};

pub fn order_functions_by_dependencies(
    syntax_tree: &SyntaxTree,
    function_calls: &FunctionCalls,
) -> OrderedFunctions {
    let dependency_graph = build_dependency_graph(syntax_tree, function_calls);
    let dependency_clusters = collect_dependency_clusters(dependency_graph);
    OrderedFunctions::from_clusters(dependency_clusters)
}

fn build_dependency_graph(
    syntax_tree: &SyntaxTree,
    function_calls: &FunctionCalls,
) -> DependencyGraph {
    let mut dependency_graph = Graph::new();
    let mut graph_indices_by_function_location = BTreeMap::new();

    for function in syntax_tree.all_functions() {
        graph_indices_by_function_location
            .entry(function.location.clone())
            .or_insert_with(|| dependency_graph.add_node(function.location));
    }

    for function in syntax_tree.all_functions() {
        let depender = graph_indices_by_function_location[&function.location];

        for branch in function.branches() {
            for expression in branch.expressions() {
                let dependee = match expression.fragment {
                    Expression::Identifier { .. } => function_calls
                        .is_call_to_user_defined_function(&expression.location)
                        .cloned(),
                    _ => expression
                        .into_local_function()
                        .map(|function| function.location),
                };

                if let Some(dependee) = dependee {
                    let dependee =
                        graph_indices_by_function_location[&dependee];
                    dependency_graph.add_edge(depender, dependee, ());
                }
            }
        }
    }

    dependency_graph
}

fn collect_dependency_clusters(
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
                    using every node from the sorted version once, to remove \
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
            syntax::SyntaxTree, Cluster, FunctionCalls, Index,
            OrderedFunctions, Tokens,
        },
        host::NoHost,
    };

    #[test]
    fn no_recursion() {
        let (syntax_tree, ordered_functions) = order_functions_by_dependencies(
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

        let f = syntax_tree.function_by_name("f").unwrap().location();
        let g = syntax_tree.function_by_name("g").unwrap().location();

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
        let (syntax_tree, ordered_functions) = order_functions_by_dependencies(
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

        let f = syntax_tree.function_by_name("f").unwrap().location();
        let g = syntax_tree.function_by_name("g").unwrap().location();

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
        let (syntax_tree, ordered_functions) = order_functions_by_dependencies(
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

        let f = syntax_tree.function_by_name("f").unwrap().location();
        let g = syntax_tree.function_by_name("g").unwrap().location();
        let h = syntax_tree.function_by_name("h").unwrap().location();

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
        let (syntax_tree, ordered_functions) = order_functions_by_dependencies(
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

        let f = syntax_tree.function_by_name("f").unwrap().location();
        let g = syntax_tree.function_by_name("g").unwrap().location();
        let h = syntax_tree.function_by_name("h").unwrap().location();

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
        let (syntax_tree, ordered_functions) = order_functions_by_dependencies(
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
            let named = syntax_tree.function_by_name(name).unwrap();
            let anonymous = named
                .into_located_function()
                .find_single_branch()
                .unwrap()
                .expressions()
                .next()
                .unwrap()
                .into_local_function()
                .map(|function| function.location)
                .unwrap();

            (named.location(), anonymous)
        });
        let g = syntax_tree.function_by_name("g").unwrap().location();

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
    ) -> (SyntaxTree, OrderedFunctions) {
        let tokens = Tokens::tokenize(input);
        let syntax_tree = SyntaxTree::parse(tokens);
        let function_calls = FunctionCalls::resolve(&syntax_tree, &NoHost);
        let ordered_functions = super::order_functions_by_dependencies(
            &syntax_tree,
            &function_calls,
        );

        (syntax_tree, ordered_functions)
    }
}
