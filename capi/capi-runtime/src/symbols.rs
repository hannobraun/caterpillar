use std::collections::BTreeMap;

use crate::InstructionAddress;

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Symbols {
    address_to_name: BTreeMap<InstructionAddress, String>,
    name_to_address: BTreeMap<String, InstructionAddress>,
}
impl Symbols {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, name: String, address: InstructionAddress) {
        self.address_to_name.insert(address, name.clone());
        self.name_to_address.insert(name, address);
    }

    pub fn resolve_address(
        &self,
        address: &InstructionAddress,
    ) -> Option<&str> {
        self.address_to_name.get(address).map(|s| s.as_str())
    }

    pub fn resolve_name(&self, name: &str) -> InstructionAddress {
        let Some(address) = self.name_to_address.get(name).copied() else {
            panic!("Can't find function `{name}`");
        };
        address
    }
}
