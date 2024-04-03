use std::collections::BTreeSet;

use super::{
    code::Code,
    compiler::compile,
    syntax::{Syntax, SyntaxElement},
};

#[derive(Debug)]
pub struct Functions {
    pub names: BTreeSet<&'static str>,
    pub inner: Vec<Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            names: BTreeSet::new(),
            inner: Vec::new(),
        }
    }

    pub fn define(&mut self, name: &'static str, f: impl FnOnce(&mut Syntax)) {
        if self.names.contains(name) {
            panic!("Can't re-define existing function `{name}`.");
        }

        let mut syntax = Vec::new();
        f(&mut Syntax::new(&mut syntax));

        self.names.insert(name);
        self.inner.push(Function { name, syntax });
    }

    pub fn print(&self) {
        for Function { name, syntax } in &self.inner {
            println!();
            println!("{name}:");

            for element in syntax {
                println!("    {element}");
            }
        }
    }

    pub fn compile(self) -> Code {
        let mut code = Code::new();

        for Function { name, syntax } in self.inner {
            compile(name, syntax, &self.names, &mut code);
        }

        code
    }
}

#[derive(Debug)]
pub struct Function {
    pub name: &'static str,
    pub syntax: Vec<SyntaxElement>,
}
