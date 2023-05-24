use std::collections::BTreeMap;

use super::syntax::SyntaxTree;

pub struct Functions {
    inner: BTreeMap<String, SyntaxTree>,
}

impl Functions {
    pub fn new() -> Functions {
        Functions {
            inner: BTreeMap::new(),
        }
    }

    pub fn define(&mut self, name: String, body: SyntaxTree) {
        self.inner.insert(name, body);
    }

    pub fn get(&self, name: &str) -> Option<SyntaxTree> {
        self.inner.get(name).cloned()
    }
}
