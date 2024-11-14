mod build_call_graph;
mod detect_changes;
mod find_divergent_functions;
mod generate_instructions;
mod infer_types;
mod mark_tail_positions;
mod parse;
mod resolve_functions;
mod resolve_most_identifiers;
mod resolve_recursive_calls;
mod tokenize;

pub use {
    build_call_graph::build_call_graph, detect_changes::detect_changes,
    find_divergent_functions::find_divergent_functions,
    generate_instructions::generate_instructions, infer_types::infer_types,
    mark_tail_positions::mark_tail_positions, parse::parse,
    resolve_functions::resolve_calls_to_user_defined_functions,
    resolve_most_identifiers::resolve_most_identifiers,
    resolve_recursive_calls::resolve_recursive_calls, tokenize::tokenize,
};
