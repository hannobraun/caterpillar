mod a_tail_position;
mod b_resolve;
mod c_clusters;
mod d_fragments;
mod e_bytecode;

pub use {
    a_tail_position::determine_tail_positions, b_resolve::resolve_identifiers,
    c_clusters::group_functions, d_fragments::generate_fragments,
    e_bytecode::generate_bytecode,
};
