use std::collections::{btree_map, BTreeMap};

use super::syntax::SyntaxTree;

pub struct Functions {
    inner: BTreeMap<(String, String), Function>,
}

impl Functions {
    pub fn new() -> Functions {
        Functions {
            inner: BTreeMap::new(),
        }
    }

    pub fn define(&mut self, module: String, name: String, body: SyntaxTree) {
        let function = Function { body };
        self.inner.insert((module, name), function);
    }

    pub fn get(&self, module: &str, name: &str) -> Option<SyntaxTree> {
        self.inner
            .get(&(module.into(), name.into()))
            .cloned()
            .map(|function| function.body)
    }
}

impl IntoIterator for Functions {
    type Item = ((String, String), Function);
    type IntoIter = btree_map::IntoIter<(String, String), Function>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

#[derive(Clone)]
pub struct Function {
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
