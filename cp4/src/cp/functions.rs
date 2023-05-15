use std::collections::{btree_map, BTreeMap};

use super::syntax::SyntaxTree;

pub struct Functions {
    inner: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<Function> {
        self.inner.get(name).cloned()
    }

    pub fn define(&mut self, name: String, module: String, body: SyntaxTree) {
        let function = Function {
            name: name.clone(),
            module,
            body,
        };
        self.inner.insert(name, function);
    }
}

impl IntoIterator for Functions {
    type Item = (String, Function);
    type IntoIter = btree_map::IntoIter<String, Function>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Clone)]
pub struct Function {
    pub module: String,
    pub name: String,
    pub body: SyntaxTree,
}
