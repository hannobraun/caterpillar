use std::collections::BTreeMap;

use crate::runtime::{self, Instructions};

#[derive(
    Clone, Debug, Eq, PartialEq, Default, serde::Deserialize, serde::Serialize,
)]
pub struct Code {
    pub instructions: Instructions,
    pub functions: BTreeMap<String, runtime::Function>,
}
