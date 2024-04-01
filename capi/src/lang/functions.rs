use std::collections::BTreeSet;

use super::syntax::{Syntax, SyntaxElement};

#[derive(Debug)]
pub struct Functions {
    pub names: BTreeSet<&'static str>,
    pub inner: Vec<(&'static str, Vec<SyntaxElement>)>,
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
        self.inner.push((name, syntax));
    }
}
