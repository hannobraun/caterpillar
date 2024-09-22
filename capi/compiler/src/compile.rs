use capi_runtime::Instructions;

use crate::{
    fragments::Fragments,
    host::Host,
    passes::{
        determine_tail_positions, generate_fragments, generate_instructions,
        parse, resolve_identifiers, tokenize,
    },
    source_map::SourceMap,
    syntax::Script,
};

pub fn tokenize_and_parse(source: &str) -> Script {
    let tokens = tokenize(source);
    parse(tokens)
}

pub fn compile<H: Host>(source: &str) -> (Fragments, Instructions, SourceMap) {
    let mut script = tokenize_and_parse(source);
    determine_tail_positions(&mut script.functions);
    resolve_identifiers::<H>(&mut script.functions);
    let fragments = generate_fragments(script.functions);
    let (instructions, source_map) = generate_instructions(fragments.clone());

    (fragments, instructions, source_map)
}
