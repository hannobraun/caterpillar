use std::collections::BTreeMap;

use super::data_stack::Value;

pub struct Bindings {
    pub inner: BTreeMap<String, Value>,
}

impl Bindings {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
}
