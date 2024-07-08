mod fragments_to_bytecode;
mod syntax_to_fragments;

pub use {
    fragments_to_bytecode::fragments_to_bytecode,
    syntax_to_fragments::script_to_fragments,
};
