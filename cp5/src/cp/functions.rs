use std::collections::{btree_map, BTreeMap};

use super::syntax::SyntaxTree;

pub struct Functions {
    inner: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Functions {
        Functions {
            inner: BTreeMap::new(),
        }
    }

    pub fn define(&mut self, module: String, name: String, body: SyntaxTree) {
        let function = Function { module, body };
        self.inner.insert(name, function);
    }

    pub fn get(&self, name: &str) -> Option<SyntaxTree> {
        self.inner.get(name).cloned().map(|function| function.body)
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
    pub body: SyntaxTree,
}

#[derive(Clone, Copy)]
pub struct Module<'r> {
    _inner: Option<&'r str>,
}

impl<'r> Module<'r> {
    pub fn none() -> Self {
        Self { _inner: None }
    }

    pub fn some(s: &'r str) -> Self {
        Self { _inner: Some(s) }
    }
}
