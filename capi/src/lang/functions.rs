use std::collections::BTreeMap;

use super::syntax::SyntaxElement;

#[derive(Debug)]
pub struct Functions {
    pub inner: BTreeMap<&'static str, Vec<SyntaxElement>>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn define(&mut self, name: &'static str, syntax: Vec<SyntaxElement>) {
        if self.inner.contains_key(name) {
            panic!("Can't re-define existing function `{name}`.");
        }

        self.inner.insert(name, syntax);
    }
}
