use std::collections::BTreeMap;

use super::Fragment;

pub struct Compiler {
    pub functions: BTreeMap<&'static str, Vec<Fragment>>,
    pub fragments: Vec<Fragment>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            functions: BTreeMap::new(),
            fragments: Vec::new(),
        }
    }
}
