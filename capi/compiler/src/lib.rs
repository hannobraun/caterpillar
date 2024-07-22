pub mod repr;
pub mod source_map;
pub mod stages;

#[cfg(test)]
mod tests;

pub fn compile<H: capi_process::Host>(
    script: crate::repr::syntax::Script,
) -> (
    crate::repr::fragments::Fragments,
    capi_process::Bytecode,
    crate::source_map::SourceMap,
) {
    let fragments = stages::script_to_fragments::<H>(script);
    let (bytecode, source_map) =
        stages::fragments_to_bytecode(fragments.clone());

    (fragments, bytecode, source_map)
}
