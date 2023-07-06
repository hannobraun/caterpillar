use std::collections::{btree_map, BTreeMap, BTreeSet};

use crate::cp::pipeline::ir::analyzer_output::AnalyzerOutput;

use super::{Function, FunctionBody, Intrinsic};

#[derive(Debug, Default)]
pub struct Functions {
    declarations: BTreeSet<String>,
    definitions: BTreeMap<String, Function>,
    updated: BTreeSet<String>,
}

impl Functions {
    pub fn new() -> Functions {
        Self::default()
    }

    pub fn register_intrinsic(
        &mut self,
        module: Module,
        name: String,
        body: Intrinsic,
    ) {
        let module = module.name();

        self.declarations.insert(name.clone());

        let function = Function {
            module,
            body: FunctionBody::Intrinsic(body),
        };
        self.definitions.insert(name.clone(), function);

        self.updated.insert(name);
    }

    pub fn declare(&mut self, name: String) {
        self.declarations.insert(name);
    }

    pub fn define(
        &mut self,
        module: Module,
        name: String,
        body: AnalyzerOutput,
    ) {
        assert!(
            self.is_declared(&name),
            "Must declare function before defining it"
        );

        let module = module.name();
        let function = Function {
            module,
            body: FunctionBody::UserDefined(body),
        };
        self.definitions.insert(name.clone(), function);

        self.updated.insert(name);
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.declarations.contains(name)
    }

    pub fn get(&self, name: &str) -> &Function {
        match self.definitions.get(name) {
            Some(function) => function,
            None => {
                let is_declared = self.is_declared(name);
                panic!(
                    "Function `{name}` not defined. Is declared: {is_declared}"
                )
            }
        }
    }

    pub fn clear_updated(&mut self) -> BTreeSet<String> {
        let updated = self.updated.clone();
        self.updated.clear();
        updated
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
