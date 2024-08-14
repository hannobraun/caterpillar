use capi_process::Instructions;

use crate::{
    fragments::Fragments,
    host::Host,
    passes::{
        determine_tail_positions, find_functions, generate_fragments,
        generate_instructions, resolve_identifiers,
    },
    source_map::SourceMap,
    syntax::Script,
};

pub fn compile<H: Host>(
    mut script: Script,
) -> (Fragments, Instructions, SourceMap) {
    determine_tail_positions(&mut script.branches);
    let mut clusters = find_functions(script.branches);
    resolve_identifiers::<H>(&mut clusters);
    let fragments = generate_fragments(clusters);
    let (instructions, source_map) =
        generate_instructions::<H>(fragments.clone());

    (fragments, instructions, source_map)
}
