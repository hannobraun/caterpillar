mod syntax_to_bytecode;
mod syntax_to_fragments;

pub use {
    syntax_to_bytecode::fragments_to_bytecode,
    syntax_to_fragments::syntax_to_fragments,
};
