use std::collections::BTreeMap;

use super::syntax::SyntaxTree;

pub struct Functions {
    inner: BTreeMap<(String, String), SyntaxTree>,
}

impl Functions {
    pub fn new() -> Functions {
        Functions {
            inner: BTreeMap::new(),
        }
    }

    pub fn define(&mut self, module: String, name: String, body: SyntaxTree) {
        self.inner.insert((module, name), body);
    }

    pub fn get(&self, module: &str, name: &str) -> Option<SyntaxTree> {
        self.inner.get(&(module.into(), name.into())).cloned()
    }
}
