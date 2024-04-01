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
        if self.inner.contains_key(name) {
            panic!("Can't re-define existing function `{name}`.");
        }

        self.inner.insert(name, address);
    }

    pub fn resolve(&self, name: &str) -> Option<usize> {
        self.inner.get(name).copied()
    }
}
