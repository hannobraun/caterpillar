use std::collections::BTreeSet;

use super::{
    code::Code,
    compiler::compile,
    syntax::{Syntax, SyntaxElement},
};

#[derive(Clone, Debug, Default)]
pub struct Functions {
    pub names: BTreeSet<String>,
    pub inner: Vec<Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: &str, f: impl FnOnce(&mut Syntax)) {
        if self.names.contains(name) {
            panic!("Can't re-define existing function `{name}`.");
        }

        let mut syntax = Vec::new();
        f(&mut Syntax::new(&mut syntax));

        self.names.insert(name.to_string());
        self.inner.push(Function {
            name: name.to_string(),
            syntax,
        });
    }

    pub fn compile(&self) -> Code {
        let mut code = Code::new();

        for Function { name, syntax } in &self.inner {
            compile(name.clone(), syntax, &self.names, &mut code);
        }

        code
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Function {
    pub name: String,
    pub syntax: Vec<SyntaxElement>,
}
