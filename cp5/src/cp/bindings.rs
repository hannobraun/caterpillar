use std::collections::BTreeMap;

use super::data_stack::Value;

#[derive(Default)]
pub struct Bindings {
    definitions: BTreeMap<String, Value>,
}

impl Bindings {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.definitions.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        self.definitions.get(name)
    }
}
