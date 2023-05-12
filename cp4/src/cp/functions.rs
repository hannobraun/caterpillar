use std::collections::BTreeMap;

use super::syntax::SyntaxTree;

pub struct Functions(pub BTreeMap<String, SyntaxTree>);

impl Functions {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}
