use std::collections::BTreeMap;

use crate::Function;

#[derive(
    Clone, Debug, Default, Eq, PartialEq, serde::Deserialize, serde::Serialize,
)]
pub struct Code {
    pub functions: BTreeMap<String, Function>,
}

impl Code {
    pub fn entry(&self) -> Option<Function> {
        self.functions.get("main").cloned()
    }
}
