pub mod fragments;
pub mod source_map;
pub mod syntax;

mod passes;

#[cfg(test)]
mod tests;

pub fn compile<H: capi_process::Host>(
    mut script: crate::syntax::Script,
) -> (
    crate::fragments::Fragments,
    capi_process::Bytecode,
    crate::source_map::SourceMap,
) {
    passes::determine_tail_positions(&mut script.functions);
    passes::resolve_identifiers::<H>(&mut script.functions);
    let clusters = passes::find_clusters(script.functions);
    let fragments = passes::generate_fragments(clusters);
    let (bytecode, source_map) = passes::generate_bytecode(fragments.clone());

    (fragments, bytecode, source_map)
}
