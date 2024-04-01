use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Symbols {
    inner: BTreeMap<&'static str, usize>,
}
impl Symbols {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }

    pub fn define(&mut self, name: &'static str, address: usize) {
        self.inner.insert(name, address);
    }

    pub fn resolve(&self, name: &str) -> usize {
        self.inner.get(name).copied().unwrap()
    }
}
