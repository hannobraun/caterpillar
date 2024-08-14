use capi_process::Instructions;

use crate::{
    fragments::Fragments,
    host::Host,
    passes::{
        determine_tail_positions, generate_fragments, generate_instructions,
        resolve_identifiers,
    },
    source_map::SourceMap,
    syntax::Script,
};

pub fn compile<H: Host>(
    mut script: Script,
) -> (Fragments, Instructions, SourceMap) {
    determine_tail_positions(&mut script.functions);
    resolve_identifiers::<H>(&mut script.functions);
    let fragments = generate_fragments(script.functions);
    let (instructions, source_map) =
        generate_instructions::<H>(fragments.clone());

    (fragments, instructions, source_map)
}
