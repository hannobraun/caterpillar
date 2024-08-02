mod a_resolve;
mod c_fragments;
mod d_bytecode;

pub use {
    a_resolve::resolve_references, c_fragments::generate_fragments,
    d_bytecode::generate_bytecode,
};
