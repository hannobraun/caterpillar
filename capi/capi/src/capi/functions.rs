use std::{collections::BTreeSet, fmt};

use super::{
    code::Code,
    compiler::compile,
    syntax::{Syntax, SyntaxElement},
};

#[derive(Clone, Debug)]
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

    pub fn compile(&self) -> Code {
        let mut code = Code::new();

        for Function { name, syntax } in &self.inner {
            compile(name, syntax, &self.names, &mut code);
        }

        code
    }
}

impl fmt::Display for Functions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for Function { name, syntax } in &self.inner {
            writeln!(f)?;
            writeln!(f, "{name}:")?;

            for element in syntax {
                writeln!(f, "    {element}")?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: &'static str,
    pub syntax: Vec<SyntaxElement>,
}
