use std::collections::{btree_map, BTreeMap, BTreeSet};

use crate::cp::expressions::Expressions;

use super::{Function, FunctionBody, IntrinsicBody};

#[derive(Debug, Default)]
pub struct Functions {
    declarations: BTreeSet<String>,
    definitions: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Functions {
        Self::default()
    }

    pub fn declare(&mut self, name: String) {
        self.declarations.insert(name);
    }

    pub fn define(&mut self, module: Module, name: String, body: Expressions) {
        let module = module.name();
        let function = Function {
            module,
            body: FunctionBody::UserDefined(body),
        };
        self.definitions.insert(name, function);
    }

    pub fn register_intrinsic(
        &mut self,
        module: Module,
        name: String,
        body: IntrinsicBody,
    ) {
        let module = module.name();
        let function = Function {
            module,
            body: FunctionBody::Intrinsic(body),
        };
        self.definitions.insert(name, function);
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.declarations.contains(name) || self.definitions.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&Function> {
        self.definitions.get(name)
    }
}

impl IntoIterator for Functions {
    type Item = (String, Function);
    type IntoIter = btree_map::IntoIter<String, Function>;

    fn into_iter(self) -> Self::IntoIter {
        self.definitions.into_iter()
    }
}

impl<'a> IntoIterator for &'a Functions {
    type Item = (&'a String, &'a Function);
    type IntoIter = btree_map::Iter<'a, String, Function>;

    fn into_iter(self) -> Self::IntoIter {
        self.definitions.iter()
    }
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
