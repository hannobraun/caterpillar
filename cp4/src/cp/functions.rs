use std::collections::{btree_map, BTreeMap};

use super::syntax::SyntaxTree;

pub struct Functions {
    functions: BTreeMap<String, SyntaxTree>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            functions: BTreeMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<SyntaxTree> {
        self.functions.get(name).cloned()
    }

    pub fn define(&mut self, name: String, body: SyntaxTree) {
        self.functions.insert(name, body);
    }
}

impl IntoIterator for Functions {
    type Item = (String, SyntaxTree);
    type IntoIter = btree_map::IntoIter<String, SyntaxTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.functions.into_iter()
    }
}
