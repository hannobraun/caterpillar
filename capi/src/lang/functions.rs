use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Functions {
    pub inner: BTreeMap<&'static str, usize>,
}
impl Functions {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
}
