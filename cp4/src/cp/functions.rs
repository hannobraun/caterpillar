use std::collections::BTreeMap;

use super::syntax::SyntaxTree;

pub struct Functions(BTreeMap<String, SyntaxTree>);

impl Functions {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn get(&self, name: &str) -> Option<SyntaxTree> {
        self.0.get(name).cloned()
    }

    pub fn define_fn(&mut self, name: String, body: SyntaxTree) {
        self.0.insert(name, body);
    }
}
