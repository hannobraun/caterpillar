pub mod repr;
pub mod source_map;
pub mod stages;

pub fn compile(
    script: &crate::repr::syntax::Script,
) -> (capi_process::Bytecode, crate::source_map::SourceMap) {
    let fragments = stages::syntax_to_fragments(script.clone());
    stages::fragments_to_bytecode(fragments)
}
