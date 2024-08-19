use capi_process::Instructions;

use crate::{
    fragments::Fragments,
    host::Host,
    passes::{
        determine_tail_positions, generate_fragments, generate_instructions,
        parse, resolve_identifiers, tokenize,
    },
    source_map::SourceMap,
};

pub fn compile<H: Host>(source: &str) -> (Fragments, Instructions, SourceMap) {
    let tokens = tokenize(source);
    let mut script = parse(tokens);
    determine_tail_positions(&mut script.functions);
    resolve_identifiers::<H>(&mut script.functions);
    let fragments = generate_fragments(script.functions);
    let (instructions, source_map) =
        generate_instructions::<H>(fragments.clone());

    (fragments, instructions, source_map)
}
