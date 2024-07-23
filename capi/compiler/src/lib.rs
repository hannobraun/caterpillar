pub mod passes;
pub mod repr;
pub mod source_map;

#[cfg(test)]
mod tests;

pub fn compile<H: capi_process::Host>(
    script: crate::repr::syntax::Script,
) -> (
    crate::repr::fragments::Fragments,
    capi_process::Bytecode,
    crate::source_map::SourceMap,
) {
    let fragments = passes::generate_fragments::<H>(script);
    let (bytecode, source_map) =
        passes::fragments_to_bytecode(fragments.clone());

    (fragments, bytecode, source_map)
}
