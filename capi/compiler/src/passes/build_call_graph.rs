use std::collections::BTreeMap;

use petgraph::{
    algo::{condensation, toposort},
    Graph,
};

use crate::code::{
    CallGraph, Cluster, Expression, Functions, Index, IndexMap, Located,
    NamedFunction,
};

pub fn build_call_graph(functions: &Functions) -> CallGraph {
    let pet_call_graph = build_pet_call_graph(functions);
    let clusters = collect_functions_into_clusters(pet_call_graph);
    CallGraph::from_clusters(clusters)
}

fn build_pet_call_graph(functions: &Functions) -> PetCallGraph {
    let mut call_graph = Graph::new();
    let mut graph_index_by_function_name = BTreeMap::new();

    for named_function in functions.named_functions() {
        let name = named_function.name.clone();
        let index = named_function.index();

        graph_index_by_function_name
            .entry(name)
            .or_insert_with(|| call_graph.add_node((named_function, index)));
    }

    for function in functions.all_functions() {
        let caller_index = {
            let named_function =
                functions.find_named_parent(&function.location).expect(
                    "All functions are either named, or defined in a named \
                    function. Just got `function` from `functions`. If it \
                    isn't a named function already, then the named function \
                    it's defined in must be in there.",
                );
            graph_index_by_function_name[&named_function.name]
        };

        for branch in function.branches() {
            for expression in branch.body() {
                if let Expression::UnresolvedIdentifier {
                    name,
                    is_known_to_be_call_to_user_defined_function: Some(_),
                    ..
                } = expression.fragment
                {
                    let callee_index = graph_index_by_function_name[name];
                    call_graph.add_edge(caller_index, callee_index, ());
                }
            }
        }
    }

    call_graph
}

fn collect_functions_into_clusters(
    call_graph: PetCallGraph,
) -> impl Iterator<Item = Cluster> + '_ {
    let make_acyclic = true;
    let mut clustered_call_graph = condensation(call_graph, make_acyclic);

    let clustered_and_sorted_call_graph = toposort(&clustered_call_graph, None)
        .expect(
            "The previous operation should have made the call graph acyclic. \
            Hence, topologically sorting the graph should not fail.",
        );

    clustered_and_sorted_call_graph
        .into_iter()
        .map(move |graph_index| {
            let function_group =
                clustered_call_graph.remove_node(graph_index).expect(
                    "Each entry in the sorted version of the call graph must \
                    correspond to exactly one node in the unsorted version. So \
                    using every node from the sorted version once, to remove
                    its respective node in the unsorted version, should always \
                    work.",
                );

            let mut functions = IndexMap::default();
            for (_, index) in function_group {
                functions.push(index);
            }

            Cluster {
                functions,
                divergent_functions: None,
                non_divergent_branches: None,
            }
        })
}

type PetCallGraph<'r> =
    Graph<(Located<&'r NamedFunction>, Index<NamedFunction>), ()>;

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
