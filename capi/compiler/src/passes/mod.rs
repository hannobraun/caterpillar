mod bytecode;
mod clusters;
mod generate_fragments;
mod resolve;
mod tail_position;

pub use {
    bytecode::generate_instructions, clusters::find_clusters,
    generate_fragments::generate_fragments, resolve::resolve_identifiers,
    tail_position::determine_tail_positions,
};
