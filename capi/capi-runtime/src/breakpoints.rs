use std::collections::BTreeMap;

use crate::InstructionAddress;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Breakpoints {
    durable: BTreeMap<InstructionAddress, bool>,
}

impl Breakpoints {
    pub fn durable_breakpoint_at(&self, address: &InstructionAddress) -> bool {
        self.durable.get(address) == Some(&true)
    }

    pub fn toggle_durable_at(&mut self, address: InstructionAddress) {
        let breakpoint = self.durable.entry(address).or_insert(false);
        *breakpoint = !*breakpoint;
    }
}
