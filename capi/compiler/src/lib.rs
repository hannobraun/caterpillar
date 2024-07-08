pub mod repr;
pub mod source_map;
pub mod stages;

pub fn compile(
    script: crate::repr::syntax::Script,
) -> (
    crate::repr::fragments::Fragments,
    capi_process::Bytecode,
    crate::source_map::SourceMap,
) {
    let fragments = stages::syntax_to_fragments(script);
    let (bytecode, source_map) =
        stages::fragments_to_bytecode(fragments.clone());

    (fragments, bytecode, source_map)
}
