mod syntax_to_bytecode;

pub fn run(
    script: &crate::syntax::Script,
) -> (capi_process::Bytecode, crate::source_map::SourceMap) {
    syntax_to_bytecode::compile(script)
}
