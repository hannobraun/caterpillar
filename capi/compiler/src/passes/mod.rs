mod clusters;
mod generate_fragments;
mod generate_instructions;
mod resolve;
mod tail_position;

pub use {
    clusters::find_clusters, generate_fragments::generate_fragments,
    generate_instructions::generate_instructions, resolve::resolve_identifiers,
    tail_position::determine_tail_positions,
};
