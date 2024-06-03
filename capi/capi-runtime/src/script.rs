use crate::{
    code::Code,
    compiler::Compiler,
    source_map::SourceMap,
    syntax::{Function, Functions, SyntaxBuilder},
    Program,
};

#[derive(Default)]
pub struct Script {
    pub functions: Functions,
}

impl Script {
    pub fn function(&mut self, name: &str, f: impl FnOnce(&mut SyntaxBuilder)) {
        self.functions.define(name, f)
    }

    pub fn compile(script: Script, entry: &str) -> Program {
        let mut code = Code::new();
        let mut source_map = SourceMap::default();

        let mut compiler = Compiler {
            functions: &script.functions.names,
            code: &mut code,
            source_map: &mut source_map,
        };

        for Function { name, syntax } in &script.functions.inner {
            compiler.compile_function(name.clone(), syntax.clone());
        }

        let entry_address = code.symbols.resolve_name(entry);

        let mut program = Program {
            functions: script.functions,
            source_map,
            ..Program::default()
        };
        program.evaluator.update(code, entry_address);
        program.entry_address = entry_address;

        program
    }
}
