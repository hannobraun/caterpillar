mod a_resolve;
mod b_fragments;
mod d_bytecode;

pub use {
    a_resolve::resolve_references, b_fragments::generate_fragments,
    d_bytecode::generate_bytecode,
};
