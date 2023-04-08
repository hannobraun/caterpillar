use std::collections::BTreeMap;

use super::expressions::ExpressionGraph;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Functions {
    registry: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            registry: BTreeMap::new(),
        }
    }

    pub fn define(&mut self, name: String, function: Function) {
        self.registry.insert(name, function);
    }

    pub fn get(&self, name: &str) -> Option<Function> {
        self.registry.get(name).cloned()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Function {
    pub body: ExpressionGraph,
}
