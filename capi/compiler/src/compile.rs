use capi_process::Instructions;

use crate::{
    fragments::Fragments,
    host::Host,
    passes::{
        determine_tail_positions, find_clusters, generate_fragments,
        generate_instructions, resolve_identifiers,
    },
    source_map::SourceMap,
    syntax::Script,
};

pub fn compile<H: Host>(
    mut script: Script,
) -> (Fragments, Instructions, SourceMap) {
    determine_tail_positions(&mut script.functions);
    let mut clusters = find_clusters(script.functions);
    resolve_identifiers::<H>(&mut clusters);
    let fragments = generate_fragments(clusters);
    let (instructions, source_map) =
        generate_instructions::<H>(fragments.clone());

    (fragments, instructions, source_map)
}
