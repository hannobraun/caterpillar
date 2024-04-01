use std::collections::BTreeMap;

use super::syntax::SyntaxElement;

#[derive(Debug)]
pub struct Functions {
    pub inner: BTreeMap<&'static str, Vec<SyntaxElement>>,
}

impl Functions {
    pub fn new() -> Self {
        Self {
            inner: BTreeMap::new(),
        }
    }
}
