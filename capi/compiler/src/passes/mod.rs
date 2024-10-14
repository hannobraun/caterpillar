mod create_call_graph;
mod detect_changes;
mod generate_fragments;
mod generate_instructions;
mod mark_recursive_calls;
mod parse;
mod resolve_functions;
mod resolve_identifiers;
mod tail_position;
mod tokenize;

pub use {
    create_call_graph::create_call_graph, detect_changes::detect_changes,
    generate_fragments::generate_fragments,
    generate_instructions::generate_instructions,
    mark_recursive_calls::mark_recursive_calls, parse::parse,
    resolve_functions::resolve_calls_to_user_defined_functions,
    resolve_identifiers::resolve_most_identifiers,
    tail_position::determine_tail_positions, tokenize::tokenize,
};
