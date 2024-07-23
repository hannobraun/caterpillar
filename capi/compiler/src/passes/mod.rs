mod a_script_to_fragments;
mod c_fragments_to_bytecode;

pub use {
    a_script_to_fragments::generate_fragments,
    c_fragments_to_bytecode::fragments_to_bytecode,
};
