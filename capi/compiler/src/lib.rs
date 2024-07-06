pub mod source_map;
pub mod stages;
pub mod syntax;

pub fn compile(
    script: &crate::syntax::Script,
) -> (capi_process::Bytecode, crate::source_map::SourceMap) {
    let fragments = stages::syntax_to_fragments(script.clone());
    stages::fragments_to_bytecode(fragments)
}
