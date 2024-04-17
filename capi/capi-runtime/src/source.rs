use crate::{
    code::Code, compiler::compile, syntax::Syntax, Function, Functions, Program,
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

        for Function { name, syntax } in &self.functions.inner {
            compile(
                name.clone(),
                syntax.clone(),
                &self.functions.names,
                &mut code,
            );
        }

        let entry = code.symbols.resolve(entry);

        let mut program = Program {
            functions: self.functions,
            ..Program::default()
        };
        program.evaluator.update(code, entry);
        program.entry = entry;

        program
    }
}
