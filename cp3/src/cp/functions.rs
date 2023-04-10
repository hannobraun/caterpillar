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
        self.registry
            .get(name)
            .and_then(
                |function| if function.test { None } else { Some(function) },
            )
            .cloned()
    }

    pub fn tests(&self) -> impl Iterator<Item = (String, Function)> + '_ {
        self.registry.iter().filter_map(|(name, function)| {
            if function.test {
                Some((name.clone(), function.clone()))
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Function {
    pub body: ExpressionGraph,
    pub test: bool,
    pub module: String,
}
