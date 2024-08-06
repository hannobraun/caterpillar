pub mod repr;
pub mod source_map;

mod passes;

#[cfg(test)]
mod tests;

pub fn compile<H: capi_process::Host>(
    mut script: crate::repr::syntax::Script,
) -> (
    crate::repr::fragments::Fragments,
    capi_process::Bytecode,
    crate::source_map::SourceMap,
) {
    passes::determine_tail_positions(&mut script.functions);
    passes::resolve_identifiers::<H>(&mut script.functions);
    let fragments = passes::generate_fragments(script.functions);
    let (bytecode, source_map) = passes::generate_bytecode(fragments.clone());

    (fragments, bytecode, source_map)
}
