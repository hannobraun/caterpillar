pub mod pipeline;
pub mod source_map;
pub mod syntax;

pub fn compile(
    script: &crate::syntax::Script,
) -> (capi_process::Bytecode, crate::source_map::SourceMap) {
    let fragments = pipeline::syntax_to_fragments(script.clone());
    dbg!(fragments);

    pipeline::compile(script)
}
