use crate::{
    code::Code, compiler::Compiler, source_map::SourceMap, syntax::Syntax,
    Function, Functions, Program,
};

#[derive(Default)]
pub struct Source {
    pub functions: Functions,
}

impl Source {
    pub fn define(&mut self, name: &str, f: impl FnOnce(&mut Syntax)) {
        self.functions.define(name, f)
    }

    pub fn compile(self, entry: &str) -> Program {
        let mut code = Code::new();
        let mut source_map = SourceMap::default();

        let mut compiler = Compiler {
            functions: &self.functions.names,
            code: &mut code,
            source_map: &mut source_map,
        };

        for Function { name, syntax } in &self.functions.inner {
            Compiler::compile_function(
                name.clone(),
                syntax.clone(),
                &mut compiler,
            );
        }

        let entry_address = code.symbols.resolve_name(entry);

        let mut program = Program {
            functions: self.functions,
            source_map,
            ..Program::default()
        };
        program.evaluator.update(code, entry_address);
        program.entry_address = entry_address;

        program
    }
}
