use std::collections::BTreeMap;

use crate::Function;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Bytecode {
    pub functions: BTreeMap<String, Function>,
}

impl Bytecode {
    pub fn entry(&self) -> Option<Function> {
        self.functions.get("main").cloned()
    }
}
