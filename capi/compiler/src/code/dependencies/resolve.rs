use std::collections::BTreeMap;

use petgraph::{
    algo::{condensation, toposort},
    Graph,
};

use crate::code::{
    syntax::{Expression, FunctionLocation, SyntaxTree},
    FunctionCalls,
};

pub fn resolve_function_dependencies(
    syntax_tree: &SyntaxTree,
    function_calls: &FunctionCalls,
) -> Vec<Vec<FunctionLocation>> {
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

    collect_dependency_clusters(dependency_graph)
}

fn collect_dependency_clusters<T>(
    dependency_graph: Graph<T, ()>,
) -> Vec<Vec<T>> {
    let make_acyclic = true;
    let mut clustered_graph = condensation(dependency_graph, make_acyclic);

    let clustered_and_sorted_graph = toposort(&clustered_graph, None).expect(
        "The previous operation should have made the call graph acyclic. \
        Hence, topologically sorting the graph should not fail.",
    );

    clustered_and_sorted_graph
        .into_iter()
        .map(move |graph_index| {
            clustered_graph.remove_node(graph_index).expect(
                "Each entry in the sorted version of the call graph must \
                correspond to exactly one node in the unsorted version. So \
                using every node from the sorted version once, to remove its \
                respective node in the unsorted version, should always work.",
            )
        })
        .collect()
}
