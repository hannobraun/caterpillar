use std::collections::BTreeSet;

use super::syntax::SyntaxElement;

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

    pub fn define(&mut self, name: &'static str, syntax: Vec<SyntaxElement>) {
        if self.names.contains(name) {
            panic!("Can't re-define existing function `{name}`.");
        }

        self.names.insert(name);
        self.inner.push((name, syntax));
    }
}
