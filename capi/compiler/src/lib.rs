pub mod passes;
pub mod repr;
pub mod source_map;

#[cfg(test)]
mod tests;

pub fn compile<H: capi_process::Host>(
    mut script: crate::repr::syntax::Script,
) -> (
    crate::repr::fragments::Fragments,
    capi_process::Bytecode,
    crate::source_map::SourceMap,
) {
    passes::resolve_references::<H>(&mut script);
    let fragments = passes::generate_fragments(script);
    let (bytecode, source_map) = passes::generate_bytecode(fragments.clone());

    (fragments, bytecode, source_map)
}
