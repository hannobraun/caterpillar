mod a_tail_position;
mod b_groups;
mod c_resolve;
mod d_fragments;
mod e_bytecode;
mod f_replace;

pub use {
    a_tail_position::determine_tail_positions, b_groups::group_functions,
    c_resolve::resolve_identifiers, d_fragments::generate_fragments,
    e_bytecode::generate_bytecode, f_replace::replace_placeholders,
};
