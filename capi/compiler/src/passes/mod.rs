mod a_resolve;
mod b_fragments;
mod c_bytecode;

pub use {
    a_resolve::resolve_references, b_fragments::generate_fragments,
    c_bytecode::generate_bytecode,
};
