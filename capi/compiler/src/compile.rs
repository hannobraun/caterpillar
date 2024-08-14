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
    script: Script,
) -> (Fragments, Instructions, SourceMap) {
    let mut functions = find_functions(script.branches);
    determine_tail_positions(&mut functions);
    resolve_identifiers::<H>(&mut functions);
    let fragments = generate_fragments(functions);
    let (instructions, source_map) =
        generate_instructions::<H>(fragments.clone());

    (fragments, instructions, source_map)
}
