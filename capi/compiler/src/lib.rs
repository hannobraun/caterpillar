pub mod repr;
pub mod source_map;
pub mod stages;

pub fn compile(
    script: crate::repr::syntax::Script,
) -> (
    crate::repr::fragments::Fragments,
    capi_process::Bytecode,
    crate::source_map::SourceMap,
    crate::source_map::SourceMap2,
) {
    let fragments = stages::syntax_to_fragments(script.clone());
    let (bytecode, source_map, source_map_2) =
        stages::fragments_to_bytecode(fragments.clone());

    (fragments, bytecode, source_map, source_map_2)
}
