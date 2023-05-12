use std::collections::BTreeMap;

use super::syntax::SyntaxTree;

pub struct Functions(pub BTreeMap<String, SyntaxTree>);

impl Functions {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(&self, name: &str) -> Option<SyntaxTree> {
        self.0.get(name).cloned()
    }
}
