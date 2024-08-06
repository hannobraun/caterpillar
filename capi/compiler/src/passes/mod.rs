mod a_tail_position;
mod b_groups;
mod c_resolve;
mod d_fragments;
mod f_bytecode;

pub use {
    a_tail_position::determine_tail_positions, b_groups::group_functions,
    c_resolve::resolve_identifiers, d_fragments::generate_fragments,
    f_bytecode::generate_bytecode,
};
