use crate::syntax::{Functions, SyntaxBuilder};

#[derive(Default)]
pub struct Script {
    pub functions: Functions,
}

impl Script {
    pub fn function(&mut self, name: &str, f: impl FnOnce(&mut SyntaxBuilder)) {
        self.functions.define(name, f)
    }
}
