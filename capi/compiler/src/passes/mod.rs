mod detect_changes;
mod generate_fragments;
mod generate_instructions;
mod group_into_clusters;
mod mark_recursive_calls;
mod parse;
mod resolve_identifiers;
mod tail_position;
mod tokenize;

pub use {
    detect_changes::detect_changes, generate_fragments::generate_fragments,
    generate_instructions::generate_instructions,
    group_into_clusters::create_call_graph,
    mark_recursive_calls::mark_recursive_calls, parse::parse,
    resolve_identifiers::resolve_identifiers,
    tail_position::determine_tail_positions, tokenize::tokenize,
};
