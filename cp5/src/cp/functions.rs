use std::collections::{btree_map, BTreeMap};

use super::pipeline::c_analyzer::Expressions;

#[derive(Debug, Default)]
pub struct Functions {
    inner: BTreeMap<String, FunctionKind>,
}

impl Functions {
    pub fn new() -> Functions {
        Self::default()
    }

    pub fn define(&mut self, module: Module, name: String, body: Expressions) {
        let module = module.name();
        let function = FunctionKind::UserDefined { module, body };
        self.inner.insert(name, function);
    }

    pub fn get(&self, name: &str) -> Option<FunctionKind> {
        self.inner.get(name).cloned()
    }
}

impl IntoIterator for Functions {
    type Item = (String, FunctionKind);
    type IntoIter = btree_map::IntoIter<String, FunctionKind>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<'a> IntoIterator for &'a Functions {
    type Item = (&'a String, &'a FunctionKind);
    type IntoIter = btree_map::Iter<'a, String, FunctionKind>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.iter()
    }
}

#[derive(Clone, Debug)]
pub enum FunctionKind {
    UserDefined { module: String, body: Expressions },
}

#[derive(Clone, Copy)]
pub struct Module<'r> {
    inner: Option<&'r str>,
}

impl<'r> Module<'r> {
    pub fn none() -> Self {
        Self { inner: None }
    }

    pub fn some(s: &'r str) -> Self {
        Self { inner: Some(s) }
    }

    pub fn name(&self) -> String {
        self.inner.unwrap_or("<root>").into()
    }
}
