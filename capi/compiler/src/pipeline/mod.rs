mod syntax_to_bytecode;
mod syntax_to_fragments;

pub use {
    syntax_to_bytecode::compile, syntax_to_fragments::syntax_to_fragments,
};

pub fn run(
    script: &crate::syntax::Script,
) -> (capi_process::Bytecode, crate::source_map::SourceMap) {
    let fragments = syntax_to_fragments::syntax_to_fragments(script.clone());
    dbg!(fragments);

    syntax_to_bytecode::compile(script)
}
