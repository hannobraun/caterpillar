mod a_tail_position;
mod b_resolve;
mod c_fragments;
mod d_bytecode;

pub use {
    a_tail_position::determine_tail_positions, b_resolve::resolve_references,
    c_fragments::generate_fragments, d_bytecode::generate_bytecode,
};
