use std::collections::BTreeMap;

use crate::InstructionAddress;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Symbols {
    inner: BTreeMap<String, InstructionAddress>,
}
impl Symbols {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, address: InstructionAddress) {
        self.inner.insert(name, address);
    }

    pub fn resolve(&self, name: &str) -> InstructionAddress {
        let Some(address) = self.inner.get(name).copied() else {
            panic!("Can't find function `{name}`");
        };
        address
    }
}
