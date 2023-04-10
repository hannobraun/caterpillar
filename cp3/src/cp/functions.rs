use std::collections::BTreeMap;

use super::expressions::ExpressionGraph;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Functions {
    functions: BTreeMap<String, Function>,
    tests: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            functions: BTreeMap::new(),
            tests: BTreeMap::new(),
        }
    }

    pub fn define_function(&mut self, name: String, function: Function) {
        self.functions.insert(name, function);
    }

    pub fn define_test(&mut self, name: String, function: Function) {
        self.tests.insert(name, function);
    }

    pub fn function(&self, name: &str) -> Option<Function> {
        self.functions.get(name).cloned()
    }

    pub fn tests(&self) -> impl Iterator<Item = (String, Function)> + '_ {
        self.tests
            .iter()
            .map(|(name, function)| (name.clone(), function.clone()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Function {
    pub body: ExpressionGraph,
    pub module: String,
}
