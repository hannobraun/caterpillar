use std::collections::{btree_map, BTreeMap};

use super::syntax::SyntaxTree;

pub struct Functions {
    inner: BTreeMap<String, SyntaxTree>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<SyntaxTree> {
        self.inner.get(name).cloned()
    }

    pub fn define(&mut self, name: String, body: SyntaxTree) {
        self.inner.insert(name, body);
    }
}

impl IntoIterator for Functions {
    type Item = (String, SyntaxTree);
    type IntoIter = btree_map::IntoIter<String, SyntaxTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}
