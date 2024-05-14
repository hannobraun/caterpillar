use std::collections::BTreeMap;

use crate::InstructionAddress;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Breakpoints {
    pub durable: BTreeMap<InstructionAddress, bool>,
}

impl Breakpoints {
    pub fn durable_breakpoint_at(&self, address: &InstructionAddress) -> bool {
        self.durable.get(address) == Some(&true)
    }
}
