use std::collections::BTreeMap;

use super::Value;

pub struct Bindings {
    inner: BTreeMap<String, Value>,
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn create(&mut self, name: String, value: Value) {
        self.inner.insert(name, value);
    }

    pub fn resolve(&self, name: &str) -> Option<Value> {
        self.inner.get(name).cloned()
    }
}
