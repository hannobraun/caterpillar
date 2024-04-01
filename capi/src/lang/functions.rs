use std::collections::{BTreeMap, BTreeSet};

use super::syntax::SyntaxElement;

#[derive(Debug)]
pub struct Functions {
    pub names: BTreeSet<&'static str>,
    pub inner: BTreeMap<&'static str, Vec<SyntaxElement>>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            names: BTreeSet::new(),
            inner: BTreeMap::new(),
        }
    }

    pub fn define(&mut self, name: &'static str, syntax: Vec<SyntaxElement>) {
        if self.names.contains(name) {
            panic!("Can't re-define existing function `{name}`.");
        }

        self.names.insert(name);
        self.inner.insert(name, syntax);
    }
}
