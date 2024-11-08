mod build_call_graph;
mod detect_changes;
mod find_divergent_functions;
mod generate_instructions;
mod infer_types;
mod mark_recursive_calls;
mod parse;
mod resolve_functions;
mod resolve_most_identifiers;
mod tail_position;
mod tokenize;

pub use {
    build_call_graph::build_call_graph, detect_changes::detect_changes,
    find_divergent_functions::find_divergent_functions,
    generate_instructions::generate_instructions, infer_types::infer_types,
    mark_recursive_calls::mark_recursive_calls, parse::parse,
    resolve_functions::resolve_calls_to_user_defined_functions,
    resolve_most_identifiers::resolve_most_identifiers,
    tail_position::determine_tail_positions, tokenize::tokenize,
};
