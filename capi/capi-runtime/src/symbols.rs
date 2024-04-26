use std::collections::BTreeMap;

use crate::InstructionAddress;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Symbols {
    inner: BTreeMap<String, InstructionAddress>,
}
impl Symbols {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, address: usize) {
        self.inner.insert(name, InstructionAddress(address));
    }

    pub fn resolve(&self, name: &str) -> InstructionAddress {
        let Some(address) = self.inner.get(name).copied() else {
            panic!("Can't find function `{name}`");
        };
        address
    }
}
