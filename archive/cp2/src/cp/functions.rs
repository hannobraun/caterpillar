use std::collections::BTreeMap;

use super::expressions::ExpressionGraph;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Functions {
    pub registry: BTreeMap<String, Function>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            registry: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Function {
    pub body: ExpressionGraph,
}
