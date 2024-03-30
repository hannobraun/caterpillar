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

    pub fn b(&mut self, name: &'static str) -> &mut Self {
        self.fragments.push(Fragment::Builtin { name });
        self
    }

    pub fn f(&mut self, name: &'static str) -> &mut Self {
        let function = self.functions.get(name).unwrap();
        self.fragments.extend(function.iter().copied());
        self
    }

    pub fn v(&mut self, value: usize) -> &mut Self {
        self.fragments.push(Fragment::Value(value));
        self
    }
}
