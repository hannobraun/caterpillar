mod build_call_graph;
mod detect_changes;
mod find_divergent_functions;
mod generate_instructions;
mod resolve_most_identifiers;
mod resolve_non_recursive_functions;
mod resolve_recursive_calls;
mod resolve_recursive_local_functions;
mod sort_non_divergent_branches;

pub use {
    build_call_graph::order_functions_by_dependencies,
    detect_changes::detect_changes,
    find_divergent_functions::find_divergent_functions,
    generate_instructions::generate_instructions,
    resolve_most_identifiers::resolve_most_identifiers,
    resolve_non_recursive_functions::resolve_non_recursive_functions,
    resolve_recursive_calls::resolve_recursive_calls,
    resolve_recursive_local_functions::resolve_recursive_local_functions,
    sort_non_divergent_branches::sort_non_divergent_branches,
};
