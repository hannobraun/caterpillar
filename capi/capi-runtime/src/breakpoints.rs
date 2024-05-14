use std::collections::BTreeMap;

use crate::InstructionAddress;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Breakpoints {
    pub durable: BTreeMap<InstructionAddress, bool>,
}
