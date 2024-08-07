mod bytecode;
mod clusters;
mod d_fragments;
mod resolve;
mod tail_position;

pub use {
    bytecode::generate_bytecode, clusters::find_clusters,
    d_fragments::generate_fragments, resolve::resolve_identifiers,
    tail_position::determine_tail_positions,
};
