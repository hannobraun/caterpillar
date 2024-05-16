use std::collections::BTreeMap;

use crate::InstructionAddress;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Symbols {
    name_to_address: BTreeMap<String, InstructionAddress>,
}
impl Symbols {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, address: InstructionAddress) {
        self.name_to_address.insert(name, address);
    }

    pub fn resolve_name(&self, name: &str) -> InstructionAddress {
        let Some(address) = self.name_to_address.get(name).copied() else {
            panic!("Can't find function `{name}`");
        };
        address
    }
}
