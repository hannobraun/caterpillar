use capi_runtime::Instructions;

use crate::{
    fragments::Fragments,
    host::Host,
    passes::{
        determine_tail_positions, generate_fragments, generate_instructions,
        parse, resolve_identifiers, tokenize,
    },
    source_map::SourceMap,
    syntax::Function,
};

pub fn tokenize_and_parse(source: &str) -> Vec<Function> {
    let tokens = tokenize(source);
    parse(tokens).functions
}

pub fn compile<H: Host>(source: &str) -> (Fragments, Instructions, SourceMap) {
    let mut functions = tokenize_and_parse(source);
    determine_tail_positions(&mut functions);
    resolve_identifiers::<H>(&mut functions);
    let fragments = generate_fragments(functions);
    let (instructions, source_map) = generate_instructions(fragments.clone());

    (fragments, instructions, source_map)
}
